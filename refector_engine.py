import os
import re

SKILLS_DIR = "./src/skills"

# Moved TEMPLATE definition
TEMPLATE_BODY = """
async fn handle_{skill_name}(payload: Value) -> DsaResult<ResultBox> {{
    let nums = payload["nums"].as_array()
        .ok_or_else(|| DsaError::ValidationError {{
            message: "nums array must be provided".to_string(),
            hint: "Add 'nums' field with an array of integers.".to_string(),
        }})?;

    // Calling the original solve logic
    let result = {struct_name}::solve(nums);

    Ok(ResultBox::success(result)
        .with_complexity(json!({{ "time": "O(N)", "space": "O(1)" }}))
        .with_description("Processed via dsaengine standardized handler."))
}}
"""

def automate_refactor():
    for root, dirs, files in os.walk(SKILLS_DIR):
        for file in files:
            if file.endswith(".rs") and file not in ["mod.rs", "dijkstra.rs"]:
                path = os.path.join(root, file)
                
                try:
                    # Use utf-8 for reading
                    with open(path, 'r', encoding="utf-8", errors="ignore") as f:
                        content = f.read()

                    struct_match = re.search(r"pub struct (\w+)", content)
                    if not struct_match: 
                        continue
                    
                    struct_name = struct_match.group(1)
                    skill_name = file.replace(".rs", "")

                    # Define the new post function
                    new_post = f"""
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {{
    match handle_{skill_name}(payload).await {{
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => e.into_response(),
    }}
}}"""
                    
                    # Replace the existing post function
                    content = re.sub(r"pub async fn post\(.*?\)\s*->\s*impl IntoResponse\s*\{.*?\}", new_post, content, flags=re.DOTALL)
                    
                    # Append the handle_ function if it doesn't exist
                    if f"async fn handle_{skill_name}" not in content:
                        content += TEMPLATE_BODY.format(skill_name=skill_name, struct_name=struct_name)

                    # Use utf-8 for writing
                    with open(path, 'w', encoding="utf-8") as f:
                        f.write(content)
                    
                    # Clean console output for Windows
                    print(f"SUCCESS: {file}")
                
                except Exception as e:
                    print(f"ERROR on {file}: {str(e)}")

if __name__ == "__main__":
    automate_refactor()