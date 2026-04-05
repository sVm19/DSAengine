use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use serde_json::{json, Value};
use std::collections::VecDeque;

/// SKILL: Course Schedule
/// CATEGORY: graphs
/// DESCRIPTION: Determines whether all courses can be completed given prerequisites,
///              and returns a valid course order — DFS on a directed graph detecting cycles.
pub struct CourseSchedule;

impl Complexity for CourseSchedule {
    fn name(&self) -> &'static str {
        "Course Schedule (Prerequisite DAG / Kahn's BFS)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(V + E) — V courses, E prerequisite edges; each node and edge processed once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(V + E) — Adjacency list and in-degree array."
    }

    fn description(&self) -> &'static str {
        "Models courses as nodes and prerequisites as directed edges; applies Kahn's BFS — if fewer than V courses are scheduled, a cycle makes completion impossible."
    }
}

impl CourseSchedule {
    /// Returns `true` if all `num_courses` can be completed given `prerequisites`.
    pub fn can_finish(num_courses: usize, prerequisites: &[(usize, usize)]) -> bool {
        Self::find_order(num_courses, prerequisites).is_some()
    }

    /// Returns a valid course completion order, or `None` if a cycle exists.
    pub fn find_order(num_courses: usize, prerequisites: &[(usize, usize)]) -> Option<Vec<usize>> {
        let mut adj = vec![Vec::new(); num_courses];
        let mut in_degree = vec![0usize; num_courses];

        for &(course, prereq) in prerequisites {
            if course < num_courses && prereq < num_courses {
                adj[prereq].push(course);
                in_degree[course] += 1;
            }
        }

        AgentLogger::log(
            AgentFeedback::Info,
            format!(
                "Course schedule: {num_courses} courses, {} prerequisite(s).",
                prerequisites.len()
            ),
        );

        let mut queue: VecDeque<usize> = (0..num_courses).filter(|&c| in_degree[c] == 0).collect();

        let mut order = Vec::with_capacity(num_courses);

        while let Some(course) = queue.pop_front() {
            order.push(course);
            for &next in &adj[course] {
                in_degree[next] -= 1;
                if in_degree[next] == 0 {
                    queue.push_back(next);
                }
            }
        }

        if order.len() < num_courses {
            AgentLogger::log(
                AgentFeedback::Warning,
                "Cycle detected — no valid course ordering exists.",
            );
            return None;
        }

        AgentLogger::log(AgentFeedback::Success, format!("Valid course order found."));
        Some(order)
    }
}


// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};

#[macros::mcp_tool(name = "course_schedule", description = "Use this for solving course schedule problems. Trigger Keywords: graph, course_schedule, shortest path, traversal. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}
