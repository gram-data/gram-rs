"""JSON interchange format codec for Pattern[Subject].

Converts between the raw dict interchange format (RawPattern/RawSubject)
and native Python Pattern[Subject] objects. The interchange format is
defined by docs/pattern.schema.json in gram-data/tree-sitter-gram.
"""
from __future__ import annotations

from typing import TYPE_CHECKING, TypedDict

from ._value import value_from_dict, value_to_dict
from ._subject import Subject
from ._pattern import Pattern

if TYPE_CHECKING:
    pass


# --- Raw types (JSON interchange format) ---

class RawSubject(TypedDict):
    identity: str
    labels: list[str]
    properties: dict[str, object]


class RawPattern(TypedDict):
    subject: RawSubject
    elements: list["RawPattern"]


# --- Runtime validation ---

def validate_payload(raw: object) -> list[RawPattern]:
    """Validate that a raw JSON payload is a list of RawPattern dicts.

    Use this on the receiving side (e.g. a client deserializing a server
    response) to guard against malformed payloads before calling
    :func:`pattern_from_dict`.

    Args:
        raw: The deserialized JSON value to validate.

    Returns:
        The same list, typed as ``list[RawPattern]``.

    Raises:
        TypeError: If *raw* is not a list of dicts with the expected shape.

    Example::

        import json
        from relateby.pattern import validate_payload, pattern_from_dict

        data = json.loads(response_body)
        patterns = [pattern_from_dict(d) for d in validate_payload(data)]
    """
    if not isinstance(raw, list):
        raise TypeError(f"Invalid pattern payload: expected a list, got {type(raw).__name__}")
    for i, item in enumerate(raw):
        if not isinstance(item, dict):
            raise TypeError(f"Invalid pattern payload: item {i} must be a dict")
        if "subject" not in item or "elements" not in item:
            raise TypeError(
                f"Invalid pattern payload: item {i} must have 'subject' and 'elements' keys"
            )
        subject = item["subject"]
        if not isinstance(subject, dict):
            raise TypeError(f"Invalid pattern payload: item {i} 'subject' must be a dict")
        for field in ("identity", "labels", "properties"):
            if field not in subject:
                raise TypeError(
                    f"Invalid pattern payload: item {i} 'subject' missing '{field}'"
                )
        if not isinstance(item["elements"], list):
            raise TypeError(f"Invalid pattern payload: item {i} 'elements' must be a list")
    return raw  # type: ignore[return-value]


# --- Raw dict → native ---

def pattern_from_dict(d: RawPattern) -> Pattern[Subject]:
    """Construct a ``Pattern[Subject]`` from a raw interchange dict.

    Expected input shape (from :func:`relateby.gram.parse_raw` or any
    producer of the JSON interchange format):

    .. code-block:: json

        {
          "subject": {
            "identity": "alice",
            "labels": ["Person"],
            "properties": {"name": "Alice"}
          },
          "elements": []
        }

    Args:
        d: A raw interchange dict, typically from :func:`validate_payload`.

    Returns:
        A native ``Pattern[Subject]`` object.

    Raises:
        ValueError: If *d* does not have the expected shape.

    Example::

        from relateby.pattern import pattern_from_dict, validate_payload
        import json

        raw = json.loads('[{"subject":{"identity":"alice","labels":["Person"],"properties":{}},"elements":[]}]')
        patterns = [pattern_from_dict(d) for d in validate_payload(raw)]
    """
    try:
        subj_data = d["subject"]
        subject = Subject(
            identity=subj_data["identity"],
            labels=set(subj_data.get("labels", [])),
            properties={
                k: value_from_dict(v)
                for k, v in subj_data.get("properties", {}).items()
            },
        )
        elements = [pattern_from_dict(e) for e in d.get("elements", [])]
        return Pattern(value=subject, elements=elements)
    except (KeyError, TypeError, ValueError) as exc:
        raise ValueError(f"Failed to decode pattern from dict: {exc}") from exc


# --- Native → raw dict ---

def pattern_to_dict(p: Pattern[Subject]) -> RawPattern:
    """Serialize a ``Pattern[Subject]`` to the raw JSON interchange dict.

    The returned dict matches the shape defined by the canonical JSON Schema
    (``docs/pattern.schema.json`` in ``gram-data/tree-sitter-gram``) and can
    be passed to :func:`json.dumps` for transport or storage.

    Args:
        p: A native ``Pattern[Subject]`` object.

    Returns:
        A ``RawPattern`` dict suitable for JSON serialization.

    Example::

        import json
        from relateby.gram import parse
        from relateby.pattern import pattern_to_dict

        patterns = parse("(alice:Person)")
        payload = json.dumps([pattern_to_dict(p) for p in patterns])
    """
    return {
        "subject": {
            "identity": p.value.identity,
            "labels": sorted(p.value.labels),
            "properties": {k: value_to_dict(v) for k, v in p.value.properties.items()},
        },
        "elements": [pattern_to_dict(e) for e in p.elements],
    }
