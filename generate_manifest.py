import os
import glob
import re
import json

SKILLS_DIR = "src/skills"

KEY_HINTS = ["nums", "numbers", "arr", "target", "edges", "adj", "source", "capacity", "weight", "values", "s1", "s2", "k", "n"]


def normalize_name(name):
    return name.replace('.', '_').replace('-', '_')


def normalize_tag(category):
    # prefer category from SKILL comments, else from file path
    tag = category.strip().replace('_', ' ').replace('-', ' ').title()
    return tag


def text_to_summary(text):
    words = re.split(r"\s+", text.strip())
    if not words:
        return "No summary available."
    if len(words) >= 10:
        return " ".join(words[:10])
    # pad to about 10 words while clamping
    if len(words) < 10:
        return " ".join(words + ["..."] * (10 - len(words)))
    return " ".join(words)


def parse_schema_from_fn(content):
    schema_keys = set()
    # Extract signature of solve or compute functions
    for m in re.finditer(r"pub\s+fn\s+(?:solve|compute|run|find|search)\s*\(([^)]*)\)", content):
        args = m.group(1)
        # push all argument names before ':'
        for arg in args.split(','):
            arg = arg.strip()
            if not arg or arg.startswith('&self') or arg.startswith('self'):
                continue
            key = arg.split(':')[0].strip()
            if key:
                schema_keys.add(key)
    return list(schema_keys)


def extract_meta(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        text = f.read()

    # op name from macros or SKILL comments
    m_macro = re.search(r'#\[macros::mcp_tool\(\s*name\s*=\s*"([^"]+)"', text)
    op_name = None
    if m_macro:
        op_name = m_macro.group(1)

    m_skill = re.search(r'///\s*SKILL:\s*(.+)', text)
    m_cat = re.search(r'///\s*CATEGORY:\s*(.+)', text)
    m_desc = re.search(r'///\s*DESCRIPTION:\s*(.+)', text)

    if not op_name and m_skill:
        op_name = m_skill.group(1).strip().lower().replace(' ', '_')

    if not op_name:
        # fallback to file stem
        base = os.path.splitext(os.path.basename(file_path))[0]
        op_name = base.replace('-', '_')

    category = (m_cat.group(1).strip() if m_cat else None) or op_name.split('.')[0]
    title = (m_skill.group(1).strip() if m_skill else op_name)
    description = (m_desc.group(1).strip() if m_desc else f"Executes the {title} skill.")

    # summary vibe 10-word
    summary = text_to_summary(re.sub(r'\s+', ' ', description))

    # Input schema in order: from fn signature or known keys in description+content
    keys = parse_schema_from_fn(text)
    if not keys:
        found = []
        for key in KEY_HINTS:
            if re.search(rf"\b{re.escape(key)}\b", text):
                found.append(key)
        keys = found

    # if still no keys, then model one from category generic
    if not keys:
        keys = ["input"]

    return {
        "operationId": normalize_name(op_name),
        "skill": op_name,
        "title": title,
        "category": category,
        "tags": [normalize_tag(category)],
        "summary": summary,
        "description": f"Select this when the user needs: {description}",
        "input_keys": keys,
    }


manifest = {
    "openapi": "3.1.0",
    "info": {
        "title": "dsaengine Discovery Manifest",
        "version": "1.0.0",
        "description": "Auto-generated OpenAPI manifest for dsaengine MCPT tools."
    },
    "paths": {},
}

for rs in glob.glob(os.path.join(SKILLS_DIR, "**", "*.rs"), recursive=True):
    meta = extract_meta(rs)
    path = f"/api/v1/{meta['category']}/{meta['skill'].split('.')[-1]}"

    schema_properties = {}
    for k in meta['input_keys']:
        schema_properties[k] = {"type": "array" if k in ["nums", "numbers", "edges", "adj", "values"] else "string"}

    manifest["paths"][path] = {
        "post": {
            "operationId": meta["operationId"],
            "summary": meta["summary"],
            "description": meta["description"],
            "tags": meta["tags"],
            "requestBody": {
                "required": True,
                "content": {
                    "application/json": {
                        "schema": {
                            "type": "object",
                            "properties": schema_properties,
                            "required": meta["input_keys"]
                        }
                    }
                }
            },
            "responses": {
                "200": {
                    "description": "Successful response",
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object"
                            }
                        }
                    }
                }
            }
        }
    }

with open("discovery_manifest.json", "w", encoding="utf-8") as f:
    json.dump(manifest, f, ensure_ascii=False, indent=2)

print(f"Generated discovery_manifest.json with {len(manifest['paths'])} paths.")
