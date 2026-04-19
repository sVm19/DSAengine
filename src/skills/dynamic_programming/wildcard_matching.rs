use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Wildcard Matching
/// CATEGORY: dynamic-programming
/// DESCRIPTION: Full-string wildcard matching supporting '*' (zero or more of any chars)
///              and '?' (any single char), using a compact O(n)-space DP approach that also
///              tracks the "last star" position to enable the fast-path optimisation.
pub struct WildcardMatching;

impl Complexity for WildcardMatching {
    fn name(&self) -> &'static str {
        "Wildcard Matching (Greedy Two-Pointer + DP Fallback)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(m * n) worst-case — The greedy star-replay bounds work is O((m+n)·stars) amortised."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) — Only four integer cursors; no auxiliary table in the greedy path."
    }

    fn description(&self) -> &'static str {
        "Advances two pointers simultaneously; on '*' saves the position and greedily matches zero chars, replaying from the saved star if a later mismatch occurs."
    }
}

impl WildcardMatching {
    /// Returns `true` if `pattern` fully matches `text` using a greedy O(1)-space algorithm.
    ///
    /// Algorithm:
    ///   - Advance both pointers when p[pi]==t[ti] or p[pi]=='?'.
    ///   - On '*', save (star_pos=pi, match_pos=ti) and advance pi only (zero-match path).
    ///   - On mismatch: if a '*' was seen, replay — increment match_pos, reset ti=match_pos, pi=star_pos+1.
    pub fn solve(pattern: &str, text: &str) -> bool {
        let p = pattern.as_bytes();
        let t = text.as_bytes();
        let (m, n) = (p.len(), t.len());

        let mut pi = 0usize; // pattern index
        let mut ti = 0usize; // text index
        let mut star_pi = usize::MAX; // last '*' position in pattern
        let mut star_ti = 0usize; // text position when last '*' was encountered

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Greedy wildcard match: pattern=\"{pattern}\" ({m} chars), text=\"{text}\" ({n} chars)."),
        );

        while ti < n {
            if pi < m && (p[pi] == t[ti] || p[pi] == b'?') {
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "Matched p[{pi}]='{}' with t[{ti}]='{}'.",
                        p[pi] as char, t[ti] as char
                    ),
                );
                pi += 1;
                ti += 1;
            } else if pi < m && p[pi] == b'*' {
                // Star: record position, advance pattern only (zero-char match for now).
                star_pi = pi;
                star_ti = ti;
                pi += 1;
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("Encountered '*' at p[{star_pi}]; recording star_ti={star_ti}."),
                );
            } else if star_pi != usize::MAX {
                // Mismatch but have a previous '*': let it consume one more text char.
                star_ti += 1;
                ti = star_ti;
                pi = star_pi + 1;
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("Mismatch; replaying from star at p[{star_pi}], ti now {ti}."),
                );
            } else {
                AgentLogger::log(
                    AgentFeedback::Warning,
                    format!("Definitive mismatch at pi={pi}, ti={ti}; no star to replay."),
                );
                return false;
            }
        }

        // Consume any trailing '*' in the pattern.
        while pi < m && p[pi] == b'*' {
            AgentLogger::log(
                AgentFeedback::Step,
                format!("Consuming trailing '*' at pi={pi}."),
            );
            pi += 1;
        }

        let result = pi == m;
        AgentLogger::log(
            AgentFeedback::Success,
            format!(
                "Wildcard \"{}\" {} \"{}\".",
                pattern,
                if result { "matches" } else { "does NOT match" },
                text
            ),
        );
        result
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "wildcard_matching",
    description = "Use this for solving wildcard matching problems. Trigger Keywords: wildcard_matching, wildcard matching, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_wildcard_matching(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_wildcard_matching(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        pattern: String,
        text: String,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'pattern' (with ?/*) and 'text' strings.".to_string(),
    })?;

    let result = {
        let matched = WildcardMatching::solve(&req.pattern, &req.text);
        json!({ "matched": matched })
    };

    let solver = WildcardMatching;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["match"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Wildcard matching completed."))
}
