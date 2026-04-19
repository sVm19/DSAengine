//! Auto-generated skill routes. DO NOT EDIT MANUALLY.
//! Regenerate with: python scratch/phase3_routes.py
//!
//! Generated: 142 skill routes across 12 categories.

use crate::skills;
use axum::routing::post;
use axum::Router;

#[derive(Clone, serde::Serialize)]
pub struct McpTool {
    pub name: String,
    pub category: String,
    pub skill: String,
    pub route: String,
    pub description: String,
}

/// Registers ALL skill routes. Feature-gated per Cargo.toml paging system.
pub fn register(router: Router) -> Router {
    let r = router;
    // Feature-gated route groups
    #[cfg(feature = "advanced")]
    let r = register_advanced(r);
    #[cfg(feature = "arrays")]
    let r = register_arrays(r);
    #[cfg(feature = "backtracking")]
    let r = register_backtracking(r);
    #[cfg(feature = "dp")]
    let r = register_dp(r);
    #[cfg(feature = "fundamentals")]
    let r = register_fundamentals(r);
    #[cfg(feature = "graphs")]
    let r = register_graphs(r);
    #[cfg(feature = "greedy")]
    let r = register_greedy(r);
    #[cfg(feature = "linked_lists")]
    let r = register_linked_lists(r);
    #[cfg(feature = "sorting")]
    let r = register_sorting(r);
    #[cfg(feature = "stacks")]
    let r = register_stacks(r);
    #[cfg(feature = "trees")]
    let r = register_trees(r);
    #[cfg(feature = "trees_adv")]
    let r = register_trees_adv(r);
    r
}

#[cfg(feature = "advanced")]
fn register_advanced(router: Router) -> Router {
    router
        .route(
            "/advanced_topics/bit_manipulation",
            post(skills::advanced_topics::bit_manipulation::post),
        )
        .route(
            "/advanced_topics/bloom_filter",
            post(skills::advanced_topics::bloom_filter::post),
        )
        .route(
            "/advanced_topics/consistent_hashing",
            post(skills::advanced_topics::consistent_hashing::post),
        )
        .route(
            "/advanced_topics/lfu_cache",
            post(skills::advanced_topics::lfu_cache::post),
        )
        .route(
            "/advanced_topics/lru_cache",
            post(skills::advanced_topics::lru_cache::post),
        )
        .route(
            "/advanced_topics/segment_tree_lazy",
            post(skills::advanced_topics::segment_tree_lazy::post),
        )
        .route(
            "/advanced_topics/skip_list",
            post(skills::advanced_topics::skip_list::post),
        )
        .route(
            "/advanced_topics/suffix_array",
            post(skills::advanced_topics::suffix_array::post),
        )
        .route(
            "/advanced_topics/trie_autocomplete",
            post(skills::advanced_topics::trie_autocomplete::post),
        )
        .route(
            "/advanced_topics/union_find",
            post(skills::advanced_topics::union_find::post),
        )
}

#[cfg(feature = "arrays")]
fn register_arrays(router: Router) -> Router {
    router
        .route(
            "/arrays_strings/anagram_detector",
            post(skills::arrays_strings::anagram_detector::post),
        )
        .route(
            "/arrays_strings/array_rotation",
            post(skills::arrays_strings::array_rotation::post),
        )
        .route(
            "/arrays_strings/boyer_moore_voting",
            post(skills::arrays_strings::boyer_moore_voting::post),
        )
        .route(
            "/arrays_strings/compression",
            post(skills::arrays_strings::compression::post),
        )
        .route(
            "/arrays_strings/container_water",
            post(skills::arrays_strings::container_water::post),
        )
        .route(
            "/arrays_strings/difference_array",
            post(skills::arrays_strings::difference_array::post),
        )
        .route(
            "/arrays_strings/dutch_national_flag",
            post(skills::arrays_strings::dutch_national_flag::post),
        )
        .route(
            "/arrays_strings/kadanes_algorithm",
            post(skills::arrays_strings::kadanes_algorithm::post),
        )
        .route(
            "/arrays_strings/kmp_search",
            post(skills::arrays_strings::kmp_search::post),
        )
        .route(
            "/arrays_strings/longest_substring",
            post(skills::arrays_strings::longest_substring::post),
        )
        .route(
            "/arrays_strings/manachers_algorithm",
            post(skills::arrays_strings::manachers_algorithm::post),
        )
        .route(
            "/arrays_strings/next_permutation",
            post(skills::arrays_strings::next_permutation::post),
        )
        .route(
            "/arrays_strings/palindrome_matcher",
            post(skills::arrays_strings::palindrome_matcher::post),
        )
        .route(
            "/arrays_strings/rabin_karp",
            post(skills::arrays_strings::rabin_karp::post),
        )
        .route(
            "/arrays_strings/rainwater_trapping",
            post(skills::arrays_strings::rainwater_trapping::post),
        )
        .route(
            "/arrays_strings/sparse_table",
            post(skills::arrays_strings::sparse_table::post),
        )
        .route(
            "/arrays_strings/string_hashing",
            post(skills::arrays_strings::string_hashing::post),
        )
        .route(
            "/arrays_strings/string_toolkit",
            post(skills::arrays_strings::string_toolkit::post),
        )
        .route(
            "/arrays_strings/subarray_sum",
            post(skills::arrays_strings::subarray_sum::post),
        )
        .route(
            "/arrays_strings/suffix_array_lite",
            post(skills::arrays_strings::suffix_array_lite::post),
        )
        .route(
            "/arrays_strings/three_sum_solver",
            post(skills::arrays_strings::three_sum_solver::post),
        )
        .route(
            "/arrays_strings/two_sum_matcher",
            post(skills::arrays_strings::two_sum_matcher::post),
        )
        .route(
            "/arrays_strings/z_algorithm",
            post(skills::arrays_strings::z_algorithm::post),
        )
}

#[cfg(feature = "backtracking")]
fn register_backtracking(router: Router) -> Router {
    router
        .route(
            "/backtracking/combinations",
            post(skills::backtracking::combinations::post),
        )
        .route(
            "/backtracking/n_queens",
            post(skills::backtracking::n_queens::post),
        )
        .route(
            "/backtracking/permutations",
            post(skills::backtracking::permutations::post),
        )
        .route(
            "/backtracking/rat_in_maze",
            post(skills::backtracking::rat_in_maze::post),
        )
        .route(
            "/backtracking/subsets",
            post(skills::backtracking::subsets::post),
        )
        .route(
            "/backtracking/sudoku_solver",
            post(skills::backtracking::sudoku_solver::post),
        )
        .route(
            "/backtracking/word_search",
            post(skills::backtracking::word_search::post),
        )
}

#[cfg(feature = "dp")]
fn register_dp(router: Router) -> Router {
    router
        .route(
            "/dynamic_programming/climbing_stairs",
            post(skills::dynamic_programming::climbing_stairs::post),
        )
        .route(
            "/dynamic_programming/coin_change",
            post(skills::dynamic_programming::coin_change::post),
        )
        .route(
            "/dynamic_programming/dp_on_trees",
            post(skills::dynamic_programming::dp_on_trees::post),
        )
        .route(
            "/dynamic_programming/edit_distance",
            post(skills::dynamic_programming::edit_distance::post),
        )
        .route(
            "/dynamic_programming/fibonacci_viz",
            post(skills::dynamic_programming::fibonacci_viz::post),
        )
        .route(
            "/dynamic_programming/house_robber",
            post(skills::dynamic_programming::house_robber::post),
        )
        .route(
            "/dynamic_programming/lcs_solver",
            post(skills::dynamic_programming::lcs_solver::post),
        )
        .route(
            "/dynamic_programming/longest_increasing_sub",
            post(skills::dynamic_programming::longest_increasing_sub::post),
        )
        .route(
            "/dynamic_programming/palindrome_partition",
            post(skills::dynamic_programming::palindrome_partition::post),
        )
        .route(
            "/dynamic_programming/pattern_matcher",
            post(skills::dynamic_programming::pattern_matcher::post),
        )
        .route(
            "/dynamic_programming/regex_matching",
            post(skills::dynamic_programming::regex_matching::post),
        )
        .route(
            "/dynamic_programming/subset_sum",
            post(skills::dynamic_programming::subset_sum::post),
        )
        .route(
            "/dynamic_programming/wildcard_matching",
            post(skills::dynamic_programming::wildcard_matching::post),
        )
        .route(
            "/dynamic_programming/word_break",
            post(skills::dynamic_programming::word_break::post),
        )
}

#[cfg(feature = "fundamentals")]
fn register_fundamentals(router: Router) -> Router {
    router
        .route(
            "/dsa_fundamentals/big_o_analyzer",
            post(skills::dsa_fundamentals::big_o_analyzer::post),
        )
        .route(
            "/dsa_fundamentals/cyclic_sort_pattern",
            post(skills::dsa_fundamentals::cyclic_sort_pattern::post),
        )
        .route(
            "/dsa_fundamentals/fast_slow_pointer",
            post(skills::dsa_fundamentals::fast_slow_pointer::post),
        )
        .route(
            "/dsa_fundamentals/in_place_reversal",
            post(skills::dsa_fundamentals::in_place_reversal::post),
        )
        .route(
            "/dsa_fundamentals/iteration_vs_recursion",
            post(skills::dsa_fundamentals::iteration_vs_recursion::post),
        )
        .route(
            "/dsa_fundamentals/memory_layout",
            post(skills::dsa_fundamentals::memory_layout::post),
        )
        .route(
            "/dsa_fundamentals/merge_intervals",
            post(skills::dsa_fundamentals::merge_intervals::post),
        )
        .route(
            "/dsa_fundamentals/recursion_tree",
            post(skills::dsa_fundamentals::recursion_tree::post),
        )
        .route(
            "/dsa_fundamentals/sliding_window_detector",
            post(skills::dsa_fundamentals::sliding_window_detector::post),
        )
        .route(
            "/dsa_fundamentals/space_calculator",
            post(skills::dsa_fundamentals::space_calculator::post),
        )
        .route(
            "/dsa_fundamentals/two_pointer_detector",
            post(skills::dsa_fundamentals::two_pointer_detector::post),
        )
}

#[cfg(feature = "graphs")]
fn register_graphs(router: Router) -> Router {
    router
        .route(
            "/graphs/alien_dictionary",
            post(skills::graphs::alien_dictionary::post),
        )
        .route(
            "/graphs/bellman_ford",
            post(skills::graphs::bellman_ford::post),
        )
        .route(
            "/graphs/bfs_generator",
            post(skills::graphs::bfs_generator::post),
        )
        .route(
            "/graphs/clone_graph",
            post(skills::graphs::clone_graph::post),
        )
        .route(
            "/graphs/course_schedule",
            post(skills::graphs::course_schedule::post),
        )
        .route(
            "/graphs/dfs_generator",
            post(skills::graphs::dfs_generator::post),
        )
        .route("/graphs/dijkstra", post(skills::graphs::dijkstra::post))
        .route(
            "/graphs/island_counter",
            post(skills::graphs::island_counter::post),
        )
        .route(
            "/graphs/max_area_island",
            post(skills::graphs::max_area_island::post),
        )
        .route(
            "/graphs/mst_kruskal_prim",
            post(skills::graphs::mst_kruskal_prim::post),
        )
        .route(
            "/graphs/rotting_oranges",
            post(skills::graphs::rotting_oranges::post),
        )
        .route(
            "/graphs/topological_sort",
            post(skills::graphs::topological_sort::post),
        )
        .route("/graphs/visualizer", post(skills::graphs::visualizer::post))
        .route(
            "/graphs/word_ladder",
            post(skills::graphs::word_ladder::post),
        )
}

#[cfg(feature = "greedy")]
fn register_greedy(router: Router) -> Router {
    router
        .route(
            "/greedy_algorithms/activity_selection",
            post(skills::greedy_algorithms::activity_selection::post),
        )
        .route(
            "/greedy_algorithms/fractional_knapsack",
            post(skills::greedy_algorithms::fractional_knapsack::post),
        )
        .route(
            "/greedy_algorithms/gas_station",
            post(skills::greedy_algorithms::gas_station::post),
        )
        .route(
            "/greedy_algorithms/huffman_coding",
            post(skills::greedy_algorithms::huffman_coding::post),
        )
        .route(
            "/greedy_algorithms/jump_game",
            post(skills::greedy_algorithms::jump_game::post),
        )
        .route(
            "/greedy_algorithms/queue_reconstruct",
            post(skills::greedy_algorithms::queue_reconstruct::post),
        )
        .route(
            "/greedy_algorithms/task_scheduler",
            post(skills::greedy_algorithms::task_scheduler::post),
        )
}

#[cfg(feature = "linked_lists")]
fn register_linked_lists(router: Router) -> Router {
    router
        .route(
            "/linked_lists/add_two_numbers",
            post(skills::linked_lists::add_two_numbers::post),
        )
        .route(
            "/linked_lists/cycle_detection",
            post(skills::linked_lists::cycle_detection::post),
        )
        .route(
            "/linked_lists/flatten_multilevel",
            post(skills::linked_lists::flatten_multilevel::post),
        )
        .route(
            "/linked_lists/intersection_finder",
            post(skills::linked_lists::intersection_finder::post),
        )
        .route(
            "/linked_lists/merge_sorted",
            post(skills::linked_lists::merge_sorted::post),
        )
        .route(
            "/linked_lists/partition_list",
            post(skills::linked_lists::partition_list::post),
        )
        .route(
            "/linked_lists/random_pointer_copy",
            post(skills::linked_lists::random_pointer_copy::post),
        )
        .route(
            "/linked_lists/remove_nth_node",
            post(skills::linked_lists::remove_nth_node::post),
        )
        .route(
            "/linked_lists/reorder_list",
            post(skills::linked_lists::reorder_list::post),
        )
        .route(
            "/linked_lists/reverse_list",
            post(skills::linked_lists::reverse_list::post),
        )
        .route(
            "/linked_lists/rotate_list",
            post(skills::linked_lists::rotate_list::post),
        )
        .route(
            "/linked_lists/visualizer",
            post(skills::linked_lists::visualizer::post),
        )
}

#[cfg(feature = "sorting")]
fn register_sorting(router: Router) -> Router {
    router
        .route(
            "/sorting_searching/binary_search_template",
            post(skills::sorting_searching::binary_search_template::post),
        )
        .route(
            "/sorting_searching/counting_sort",
            post(skills::sorting_searching::counting_sort::post),
        )
        .route(
            "/sorting_searching/matrix_search",
            post(skills::sorting_searching::matrix_search::post),
        )
        .route(
            "/sorting_searching/merge_sort",
            post(skills::sorting_searching::merge_sort::post),
        )
        .route(
            "/sorting_searching/peak_finder",
            post(skills::sorting_searching::peak_finder::post),
        )
        .route(
            "/sorting_searching/quick_sort",
            post(skills::sorting_searching::quick_sort::post),
        )
        .route(
            "/sorting_searching/rotated_search",
            post(skills::sorting_searching::rotated_search::post),
        )
        .route(
            "/sorting_searching/visualizer",
            post(skills::sorting_searching::visualizer::post),
        )
}

#[cfg(feature = "stacks")]
fn register_stacks(router: Router) -> Router {
    router
        .route(
            "/stacks_queues/asteroid_collision",
            post(skills::stacks_queues::asteroid_collision::post),
        )
        .route(
            "/stacks_queues/daily_temperatures",
            post(skills::stacks_queues::daily_temperatures::post),
        )
        .route(
            "/stacks_queues/histogram_rectangle",
            post(skills::stacks_queues::histogram_rectangle::post),
        )
        .route(
            "/stacks_queues/min_stack",
            post(skills::stacks_queues::min_stack::post),
        )
        .route(
            "/stacks_queues/next_greater",
            post(skills::stacks_queues::next_greater::post),
        )
        .route(
            "/stacks_queues/queue_via_stacks",
            post(skills::stacks_queues::queue_via_stacks::post),
        )
        .route(
            "/stacks_queues/reverse_polish",
            post(skills::stacks_queues::reverse_polish::post),
        )
        .route(
            "/stacks_queues/stack_via_queues",
            post(skills::stacks_queues::stack_via_queues::post),
        )
        .route(
            "/stacks_queues/valid_parentheses",
            post(skills::stacks_queues::valid_parentheses::post),
        )
        .route(
            "/stacks_queues/visualizer",
            post(skills::stacks_queues::visualizer::post),
        )
        .route(
            "/stacks_queues/window_maximum",
            post(skills::stacks_queues::window_maximum::post),
        )
}

#[cfg(feature = "trees")]
fn register_trees(router: Router) -> Router {
    router
        .route(
            "/trees_binary/balance_checker",
            post(skills::trees_binary::balance_checker::post),
        )
        .route(
            "/trees_binary/bst_validator",
            post(skills::trees_binary::bst_validator::post),
        )
        .route(
            "/trees_binary/construct_from_traversal",
            post(skills::trees_binary::construct_from_traversal::post),
        )
        .route(
            "/trees_binary/height_calc",
            post(skills::trees_binary::height_calc::post),
        )
        .route(
            "/trees_binary/invert_tree",
            post(skills::trees_binary::invert_tree::post),
        )
        .route(
            "/trees_binary/lca_finder",
            post(skills::trees_binary::lca_finder::post),
        )
        .route(
            "/trees_binary/max_path_sum",
            post(skills::trees_binary::max_path_sum::post),
        )
        .route(
            "/trees_binary/path_sum",
            post(skills::trees_binary::path_sum::post),
        )
        .route(
            "/trees_binary/same_tree",
            post(skills::trees_binary::same_tree::post),
        )
        .route(
            "/trees_binary/serialization",
            post(skills::trees_binary::serialization::post),
        )
        .route(
            "/trees_binary/subtree_validator",
            post(skills::trees_binary::subtree_validator::post),
        )
        .route(
            "/trees_binary/symmetric_checker",
            post(skills::trees_binary::symmetric_checker::post),
        )
        .route(
            "/trees_binary/traversals",
            post(skills::trees_binary::traversals::post),
        )
        .route(
            "/trees_binary/visualizer",
            post(skills::trees_binary::visualizer::post),
        )
}

#[cfg(feature = "trees_adv")]
fn register_trees_adv(router: Router) -> Router {
    router
        .route(
            "/trees_advanced/avl_tree",
            post(skills::trees_advanced::avl_tree::post),
        )
        .route(
            "/trees_advanced/b_tree_index",
            post(skills::trees_advanced::b_tree_index::post),
        )
        .route(
            "/trees_advanced/fenwick_tree",
            post(skills::trees_advanced::fenwick_tree::post),
        )
        .route(
            "/trees_advanced/heap_priority_queue",
            post(skills::trees_advanced::heap_priority_queue::post),
        )
        .route(
            "/trees_advanced/median_stream",
            post(skills::trees_advanced::median_stream::post),
        )
        .route(
            "/trees_advanced/red_black_tree",
            post(skills::trees_advanced::red_black_tree::post),
        )
        .route(
            "/trees_advanced/segment_tree_builder",
            post(skills::trees_advanced::segment_tree_builder::post),
        )
        .route(
            "/trees_advanced/segment_tree_query",
            post(skills::trees_advanced::segment_tree_query::post),
        )
        .route(
            "/trees_advanced/top_k_elements",
            post(skills::trees_advanced::top_k_elements::post),
        )
        .route(
            "/trees_advanced/trie_impl",
            post(skills::trees_advanced::trie_impl::post),
        )
        .route(
            "/trees_advanced/trie_visualizer",
            post(skills::trees_advanced::trie_visualizer::post),
        )
}

/// Returns metadata for all registered tools (for MCP tools/list).
pub fn all_tools() -> Vec<McpTool> {
    let mut tools: Vec<McpTool> = Vec::new();
    #[cfg(feature = "advanced")]
    {
        tools.push(McpTool {
            name: format!(
                "advanced_topics.{}",
                skills::advanced_topics::bit_manipulation::TOOL_NAME
            ),
            category: "advanced_topics".to_string(),
            skill: skills::advanced_topics::bit_manipulation::TOOL_NAME.to_string(),
            route: "/api/v1/advanced_topics/bit_manipulation".to_string(),
            description: skills::advanced_topics::bit_manipulation::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "advanced_topics.{}",
                skills::advanced_topics::bloom_filter::TOOL_NAME
            ),
            category: "advanced_topics".to_string(),
            skill: skills::advanced_topics::bloom_filter::TOOL_NAME.to_string(),
            route: "/api/v1/advanced_topics/bloom_filter".to_string(),
            description: skills::advanced_topics::bloom_filter::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "advanced_topics.{}",
                skills::advanced_topics::consistent_hashing::TOOL_NAME
            ),
            category: "advanced_topics".to_string(),
            skill: skills::advanced_topics::consistent_hashing::TOOL_NAME.to_string(),
            route: "/api/v1/advanced_topics/consistent_hashing".to_string(),
            description: skills::advanced_topics::consistent_hashing::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "advanced_topics.{}",
                skills::advanced_topics::lfu_cache::TOOL_NAME
            ),
            category: "advanced_topics".to_string(),
            skill: skills::advanced_topics::lfu_cache::TOOL_NAME.to_string(),
            route: "/api/v1/advanced_topics/lfu_cache".to_string(),
            description: skills::advanced_topics::lfu_cache::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "advanced_topics.{}",
                skills::advanced_topics::lru_cache::TOOL_NAME
            ),
            category: "advanced_topics".to_string(),
            skill: skills::advanced_topics::lru_cache::TOOL_NAME.to_string(),
            route: "/api/v1/advanced_topics/lru_cache".to_string(),
            description: skills::advanced_topics::lru_cache::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "advanced_topics.{}",
                skills::advanced_topics::segment_tree_lazy::TOOL_NAME
            ),
            category: "advanced_topics".to_string(),
            skill: skills::advanced_topics::segment_tree_lazy::TOOL_NAME.to_string(),
            route: "/api/v1/advanced_topics/segment_tree_lazy".to_string(),
            description: skills::advanced_topics::segment_tree_lazy::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "advanced_topics.{}",
                skills::advanced_topics::skip_list::TOOL_NAME
            ),
            category: "advanced_topics".to_string(),
            skill: skills::advanced_topics::skip_list::TOOL_NAME.to_string(),
            route: "/api/v1/advanced_topics/skip_list".to_string(),
            description: skills::advanced_topics::skip_list::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "advanced_topics.{}",
                skills::advanced_topics::suffix_array::TOOL_NAME
            ),
            category: "advanced_topics".to_string(),
            skill: skills::advanced_topics::suffix_array::TOOL_NAME.to_string(),
            route: "/api/v1/advanced_topics/suffix_array".to_string(),
            description: skills::advanced_topics::suffix_array::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "advanced_topics.{}",
                skills::advanced_topics::trie_autocomplete::TOOL_NAME
            ),
            category: "advanced_topics".to_string(),
            skill: skills::advanced_topics::trie_autocomplete::TOOL_NAME.to_string(),
            route: "/api/v1/advanced_topics/trie_autocomplete".to_string(),
            description: skills::advanced_topics::trie_autocomplete::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "advanced_topics.{}",
                skills::advanced_topics::union_find::TOOL_NAME
            ),
            category: "advanced_topics".to_string(),
            skill: skills::advanced_topics::union_find::TOOL_NAME.to_string(),
            route: "/api/v1/advanced_topics/union_find".to_string(),
            description: skills::advanced_topics::union_find::TOOL_DESCRIPTION.to_string(),
        });
    }
    #[cfg(feature = "arrays")]
    {
        tools.push(McpTool {
            name: format!(
                "arrays_strings.{}",
                skills::arrays_strings::anagram_detector::TOOL_NAME
            ),
            category: "arrays_strings".to_string(),
            skill: skills::arrays_strings::anagram_detector::TOOL_NAME.to_string(),
            route: "/api/v1/arrays_strings/anagram_detector".to_string(),
            description: skills::arrays_strings::anagram_detector::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "arrays_strings.{}",
                skills::arrays_strings::array_rotation::TOOL_NAME
            ),
            category: "arrays_strings".to_string(),
            skill: skills::arrays_strings::array_rotation::TOOL_NAME.to_string(),
            route: "/api/v1/arrays_strings/array_rotation".to_string(),
            description: skills::arrays_strings::array_rotation::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "arrays_strings.{}",
                skills::arrays_strings::boyer_moore_voting::TOOL_NAME
            ),
            category: "arrays_strings".to_string(),
            skill: skills::arrays_strings::boyer_moore_voting::TOOL_NAME.to_string(),
            route: "/api/v1/arrays_strings/boyer_moore_voting".to_string(),
            description: skills::arrays_strings::boyer_moore_voting::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "arrays_strings.{}",
                skills::arrays_strings::compression::TOOL_NAME
            ),
            category: "arrays_strings".to_string(),
            skill: skills::arrays_strings::compression::TOOL_NAME.to_string(),
            route: "/api/v1/arrays_strings/compression".to_string(),
            description: skills::arrays_strings::compression::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "arrays_strings.{}",
                skills::arrays_strings::container_water::TOOL_NAME
            ),
            category: "arrays_strings".to_string(),
            skill: skills::arrays_strings::container_water::TOOL_NAME.to_string(),
            route: "/api/v1/arrays_strings/container_water".to_string(),
            description: skills::arrays_strings::container_water::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "arrays_strings.{}",
                skills::arrays_strings::difference_array::TOOL_NAME
            ),
            category: "arrays_strings".to_string(),
            skill: skills::arrays_strings::difference_array::TOOL_NAME.to_string(),
            route: "/api/v1/arrays_strings/difference_array".to_string(),
            description: skills::arrays_strings::difference_array::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "arrays_strings.{}",
                skills::arrays_strings::dutch_national_flag::TOOL_NAME
            ),
            category: "arrays_strings".to_string(),
            skill: skills::arrays_strings::dutch_national_flag::TOOL_NAME.to_string(),
            route: "/api/v1/arrays_strings/dutch_national_flag".to_string(),
            description: skills::arrays_strings::dutch_national_flag::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "arrays_strings.{}",
                skills::arrays_strings::kadanes_algorithm::TOOL_NAME
            ),
            category: "arrays_strings".to_string(),
            skill: skills::arrays_strings::kadanes_algorithm::TOOL_NAME.to_string(),
            route: "/api/v1/arrays_strings/kadanes_algorithm".to_string(),
            description: skills::arrays_strings::kadanes_algorithm::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "arrays_strings.{}",
                skills::arrays_strings::kmp_search::TOOL_NAME
            ),
            category: "arrays_strings".to_string(),
            skill: skills::arrays_strings::kmp_search::TOOL_NAME.to_string(),
            route: "/api/v1/arrays_strings/kmp_search".to_string(),
            description: skills::arrays_strings::kmp_search::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "arrays_strings.{}",
                skills::arrays_strings::longest_substring::TOOL_NAME
            ),
            category: "arrays_strings".to_string(),
            skill: skills::arrays_strings::longest_substring::TOOL_NAME.to_string(),
            route: "/api/v1/arrays_strings/longest_substring".to_string(),
            description: skills::arrays_strings::longest_substring::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "arrays_strings.{}",
                skills::arrays_strings::manachers_algorithm::TOOL_NAME
            ),
            category: "arrays_strings".to_string(),
            skill: skills::arrays_strings::manachers_algorithm::TOOL_NAME.to_string(),
            route: "/api/v1/arrays_strings/manachers_algorithm".to_string(),
            description: skills::arrays_strings::manachers_algorithm::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "arrays_strings.{}",
                skills::arrays_strings::next_permutation::TOOL_NAME
            ),
            category: "arrays_strings".to_string(),
            skill: skills::arrays_strings::next_permutation::TOOL_NAME.to_string(),
            route: "/api/v1/arrays_strings/next_permutation".to_string(),
            description: skills::arrays_strings::next_permutation::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "arrays_strings.{}",
                skills::arrays_strings::palindrome_matcher::TOOL_NAME
            ),
            category: "arrays_strings".to_string(),
            skill: skills::arrays_strings::palindrome_matcher::TOOL_NAME.to_string(),
            route: "/api/v1/arrays_strings/palindrome_matcher".to_string(),
            description: skills::arrays_strings::palindrome_matcher::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "arrays_strings.{}",
                skills::arrays_strings::rabin_karp::TOOL_NAME
            ),
            category: "arrays_strings".to_string(),
            skill: skills::arrays_strings::rabin_karp::TOOL_NAME.to_string(),
            route: "/api/v1/arrays_strings/rabin_karp".to_string(),
            description: skills::arrays_strings::rabin_karp::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "arrays_strings.{}",
                skills::arrays_strings::rainwater_trapping::TOOL_NAME
            ),
            category: "arrays_strings".to_string(),
            skill: skills::arrays_strings::rainwater_trapping::TOOL_NAME.to_string(),
            route: "/api/v1/arrays_strings/rainwater_trapping".to_string(),
            description: skills::arrays_strings::rainwater_trapping::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "arrays_strings.{}",
                skills::arrays_strings::sparse_table::TOOL_NAME
            ),
            category: "arrays_strings".to_string(),
            skill: skills::arrays_strings::sparse_table::TOOL_NAME.to_string(),
            route: "/api/v1/arrays_strings/sparse_table".to_string(),
            description: skills::arrays_strings::sparse_table::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "arrays_strings.{}",
                skills::arrays_strings::string_hashing::TOOL_NAME
            ),
            category: "arrays_strings".to_string(),
            skill: skills::arrays_strings::string_hashing::TOOL_NAME.to_string(),
            route: "/api/v1/arrays_strings/string_hashing".to_string(),
            description: skills::arrays_strings::string_hashing::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "arrays_strings.{}",
                skills::arrays_strings::string_toolkit::TOOL_NAME
            ),
            category: "arrays_strings".to_string(),
            skill: skills::arrays_strings::string_toolkit::TOOL_NAME.to_string(),
            route: "/api/v1/arrays_strings/string_toolkit".to_string(),
            description: skills::arrays_strings::string_toolkit::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "arrays_strings.{}",
                skills::arrays_strings::subarray_sum::TOOL_NAME
            ),
            category: "arrays_strings".to_string(),
            skill: skills::arrays_strings::subarray_sum::TOOL_NAME.to_string(),
            route: "/api/v1/arrays_strings/subarray_sum".to_string(),
            description: skills::arrays_strings::subarray_sum::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "arrays_strings.{}",
                skills::arrays_strings::suffix_array_lite::TOOL_NAME
            ),
            category: "arrays_strings".to_string(),
            skill: skills::arrays_strings::suffix_array_lite::TOOL_NAME.to_string(),
            route: "/api/v1/arrays_strings/suffix_array_lite".to_string(),
            description: skills::arrays_strings::suffix_array_lite::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "arrays_strings.{}",
                skills::arrays_strings::three_sum_solver::TOOL_NAME
            ),
            category: "arrays_strings".to_string(),
            skill: skills::arrays_strings::three_sum_solver::TOOL_NAME.to_string(),
            route: "/api/v1/arrays_strings/three_sum_solver".to_string(),
            description: skills::arrays_strings::three_sum_solver::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "arrays_strings.{}",
                skills::arrays_strings::two_sum_matcher::TOOL_NAME
            ),
            category: "arrays_strings".to_string(),
            skill: skills::arrays_strings::two_sum_matcher::TOOL_NAME.to_string(),
            route: "/api/v1/arrays_strings/two_sum_matcher".to_string(),
            description: skills::arrays_strings::two_sum_matcher::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "arrays_strings.{}",
                skills::arrays_strings::z_algorithm::TOOL_NAME
            ),
            category: "arrays_strings".to_string(),
            skill: skills::arrays_strings::z_algorithm::TOOL_NAME.to_string(),
            route: "/api/v1/arrays_strings/z_algorithm".to_string(),
            description: skills::arrays_strings::z_algorithm::TOOL_DESCRIPTION.to_string(),
        });
    }
    #[cfg(feature = "backtracking")]
    {
        tools.push(McpTool {
            name: format!(
                "backtracking.{}",
                skills::backtracking::combinations::TOOL_NAME
            ),
            category: "backtracking".to_string(),
            skill: skills::backtracking::combinations::TOOL_NAME.to_string(),
            route: "/api/v1/backtracking/combinations".to_string(),
            description: skills::backtracking::combinations::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!("backtracking.{}", skills::backtracking::n_queens::TOOL_NAME),
            category: "backtracking".to_string(),
            skill: skills::backtracking::n_queens::TOOL_NAME.to_string(),
            route: "/api/v1/backtracking/n_queens".to_string(),
            description: skills::backtracking::n_queens::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "backtracking.{}",
                skills::backtracking::permutations::TOOL_NAME
            ),
            category: "backtracking".to_string(),
            skill: skills::backtracking::permutations::TOOL_NAME.to_string(),
            route: "/api/v1/backtracking/permutations".to_string(),
            description: skills::backtracking::permutations::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "backtracking.{}",
                skills::backtracking::rat_in_maze::TOOL_NAME
            ),
            category: "backtracking".to_string(),
            skill: skills::backtracking::rat_in_maze::TOOL_NAME.to_string(),
            route: "/api/v1/backtracking/rat_in_maze".to_string(),
            description: skills::backtracking::rat_in_maze::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!("backtracking.{}", skills::backtracking::subsets::TOOL_NAME),
            category: "backtracking".to_string(),
            skill: skills::backtracking::subsets::TOOL_NAME.to_string(),
            route: "/api/v1/backtracking/subsets".to_string(),
            description: skills::backtracking::subsets::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "backtracking.{}",
                skills::backtracking::sudoku_solver::TOOL_NAME
            ),
            category: "backtracking".to_string(),
            skill: skills::backtracking::sudoku_solver::TOOL_NAME.to_string(),
            route: "/api/v1/backtracking/sudoku_solver".to_string(),
            description: skills::backtracking::sudoku_solver::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "backtracking.{}",
                skills::backtracking::word_search::TOOL_NAME
            ),
            category: "backtracking".to_string(),
            skill: skills::backtracking::word_search::TOOL_NAME.to_string(),
            route: "/api/v1/backtracking/word_search".to_string(),
            description: skills::backtracking::word_search::TOOL_DESCRIPTION.to_string(),
        });
    }
    #[cfg(feature = "dp")]
    {
        tools.push(McpTool {
            name: format!(
                "dynamic_programming.{}",
                skills::dynamic_programming::climbing_stairs::TOOL_NAME
            ),
            category: "dynamic_programming".to_string(),
            skill: skills::dynamic_programming::climbing_stairs::TOOL_NAME.to_string(),
            route: "/api/v1/dynamic_programming/climbing_stairs".to_string(),
            description: skills::dynamic_programming::climbing_stairs::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "dynamic_programming.{}",
                skills::dynamic_programming::coin_change::TOOL_NAME
            ),
            category: "dynamic_programming".to_string(),
            skill: skills::dynamic_programming::coin_change::TOOL_NAME.to_string(),
            route: "/api/v1/dynamic_programming/coin_change".to_string(),
            description: skills::dynamic_programming::coin_change::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "dynamic_programming.{}",
                skills::dynamic_programming::dp_on_trees::TOOL_NAME
            ),
            category: "dynamic_programming".to_string(),
            skill: skills::dynamic_programming::dp_on_trees::TOOL_NAME.to_string(),
            route: "/api/v1/dynamic_programming/dp_on_trees".to_string(),
            description: skills::dynamic_programming::dp_on_trees::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "dynamic_programming.{}",
                skills::dynamic_programming::edit_distance::TOOL_NAME
            ),
            category: "dynamic_programming".to_string(),
            skill: skills::dynamic_programming::edit_distance::TOOL_NAME.to_string(),
            route: "/api/v1/dynamic_programming/edit_distance".to_string(),
            description: skills::dynamic_programming::edit_distance::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "dynamic_programming.{}",
                skills::dynamic_programming::fibonacci_viz::TOOL_NAME
            ),
            category: "dynamic_programming".to_string(),
            skill: skills::dynamic_programming::fibonacci_viz::TOOL_NAME.to_string(),
            route: "/api/v1/dynamic_programming/fibonacci_viz".to_string(),
            description: skills::dynamic_programming::fibonacci_viz::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "dynamic_programming.{}",
                skills::dynamic_programming::house_robber::TOOL_NAME
            ),
            category: "dynamic_programming".to_string(),
            skill: skills::dynamic_programming::house_robber::TOOL_NAME.to_string(),
            route: "/api/v1/dynamic_programming/house_robber".to_string(),
            description: skills::dynamic_programming::house_robber::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "dynamic_programming.{}",
                skills::dynamic_programming::lcs_solver::TOOL_NAME
            ),
            category: "dynamic_programming".to_string(),
            skill: skills::dynamic_programming::lcs_solver::TOOL_NAME.to_string(),
            route: "/api/v1/dynamic_programming/lcs_solver".to_string(),
            description: skills::dynamic_programming::lcs_solver::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "dynamic_programming.{}",
                skills::dynamic_programming::longest_increasing_sub::TOOL_NAME
            ),
            category: "dynamic_programming".to_string(),
            skill: skills::dynamic_programming::longest_increasing_sub::TOOL_NAME.to_string(),
            route: "/api/v1/dynamic_programming/longest_increasing_sub".to_string(),
            description: skills::dynamic_programming::longest_increasing_sub::TOOL_DESCRIPTION
                .to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "dynamic_programming.{}",
                skills::dynamic_programming::palindrome_partition::TOOL_NAME
            ),
            category: "dynamic_programming".to_string(),
            skill: skills::dynamic_programming::palindrome_partition::TOOL_NAME.to_string(),
            route: "/api/v1/dynamic_programming/palindrome_partition".to_string(),
            description: skills::dynamic_programming::palindrome_partition::TOOL_DESCRIPTION
                .to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "dynamic_programming.{}",
                skills::dynamic_programming::pattern_matcher::TOOL_NAME
            ),
            category: "dynamic_programming".to_string(),
            skill: skills::dynamic_programming::pattern_matcher::TOOL_NAME.to_string(),
            route: "/api/v1/dynamic_programming/pattern_matcher".to_string(),
            description: skills::dynamic_programming::pattern_matcher::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "dynamic_programming.{}",
                skills::dynamic_programming::regex_matching::TOOL_NAME
            ),
            category: "dynamic_programming".to_string(),
            skill: skills::dynamic_programming::regex_matching::TOOL_NAME.to_string(),
            route: "/api/v1/dynamic_programming/regex_matching".to_string(),
            description: skills::dynamic_programming::regex_matching::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "dynamic_programming.{}",
                skills::dynamic_programming::subset_sum::TOOL_NAME
            ),
            category: "dynamic_programming".to_string(),
            skill: skills::dynamic_programming::subset_sum::TOOL_NAME.to_string(),
            route: "/api/v1/dynamic_programming/subset_sum".to_string(),
            description: skills::dynamic_programming::subset_sum::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "dynamic_programming.{}",
                skills::dynamic_programming::wildcard_matching::TOOL_NAME
            ),
            category: "dynamic_programming".to_string(),
            skill: skills::dynamic_programming::wildcard_matching::TOOL_NAME.to_string(),
            route: "/api/v1/dynamic_programming/wildcard_matching".to_string(),
            description: skills::dynamic_programming::wildcard_matching::TOOL_DESCRIPTION
                .to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "dynamic_programming.{}",
                skills::dynamic_programming::word_break::TOOL_NAME
            ),
            category: "dynamic_programming".to_string(),
            skill: skills::dynamic_programming::word_break::TOOL_NAME.to_string(),
            route: "/api/v1/dynamic_programming/word_break".to_string(),
            description: skills::dynamic_programming::word_break::TOOL_DESCRIPTION.to_string(),
        });
    }
    #[cfg(feature = "fundamentals")]
    {
        tools.push(McpTool {
            name: format!(
                "dsa_fundamentals.{}",
                skills::dsa_fundamentals::big_o_analyzer::TOOL_NAME
            ),
            category: "dsa_fundamentals".to_string(),
            skill: skills::dsa_fundamentals::big_o_analyzer::TOOL_NAME.to_string(),
            route: "/api/v1/dsa_fundamentals/big_o_analyzer".to_string(),
            description: skills::dsa_fundamentals::big_o_analyzer::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "dsa_fundamentals.{}",
                skills::dsa_fundamentals::cyclic_sort_pattern::TOOL_NAME
            ),
            category: "dsa_fundamentals".to_string(),
            skill: skills::dsa_fundamentals::cyclic_sort_pattern::TOOL_NAME.to_string(),
            route: "/api/v1/dsa_fundamentals/cyclic_sort_pattern".to_string(),
            description: skills::dsa_fundamentals::cyclic_sort_pattern::TOOL_DESCRIPTION
                .to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "dsa_fundamentals.{}",
                skills::dsa_fundamentals::fast_slow_pointer::TOOL_NAME
            ),
            category: "dsa_fundamentals".to_string(),
            skill: skills::dsa_fundamentals::fast_slow_pointer::TOOL_NAME.to_string(),
            route: "/api/v1/dsa_fundamentals/fast_slow_pointer".to_string(),
            description: skills::dsa_fundamentals::fast_slow_pointer::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "dsa_fundamentals.{}",
                skills::dsa_fundamentals::in_place_reversal::TOOL_NAME
            ),
            category: "dsa_fundamentals".to_string(),
            skill: skills::dsa_fundamentals::in_place_reversal::TOOL_NAME.to_string(),
            route: "/api/v1/dsa_fundamentals/in_place_reversal".to_string(),
            description: skills::dsa_fundamentals::in_place_reversal::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "dsa_fundamentals.{}",
                skills::dsa_fundamentals::iteration_vs_recursion::TOOL_NAME
            ),
            category: "dsa_fundamentals".to_string(),
            skill: skills::dsa_fundamentals::iteration_vs_recursion::TOOL_NAME.to_string(),
            route: "/api/v1/dsa_fundamentals/iteration_vs_recursion".to_string(),
            description: skills::dsa_fundamentals::iteration_vs_recursion::TOOL_DESCRIPTION
                .to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "dsa_fundamentals.{}",
                skills::dsa_fundamentals::memory_layout::TOOL_NAME
            ),
            category: "dsa_fundamentals".to_string(),
            skill: skills::dsa_fundamentals::memory_layout::TOOL_NAME.to_string(),
            route: "/api/v1/dsa_fundamentals/memory_layout".to_string(),
            description: skills::dsa_fundamentals::memory_layout::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "dsa_fundamentals.{}",
                skills::dsa_fundamentals::merge_intervals::TOOL_NAME
            ),
            category: "dsa_fundamentals".to_string(),
            skill: skills::dsa_fundamentals::merge_intervals::TOOL_NAME.to_string(),
            route: "/api/v1/dsa_fundamentals/merge_intervals".to_string(),
            description: skills::dsa_fundamentals::merge_intervals::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "dsa_fundamentals.{}",
                skills::dsa_fundamentals::recursion_tree::TOOL_NAME
            ),
            category: "dsa_fundamentals".to_string(),
            skill: skills::dsa_fundamentals::recursion_tree::TOOL_NAME.to_string(),
            route: "/api/v1/dsa_fundamentals/recursion_tree".to_string(),
            description: skills::dsa_fundamentals::recursion_tree::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "dsa_fundamentals.{}",
                skills::dsa_fundamentals::sliding_window_detector::TOOL_NAME
            ),
            category: "dsa_fundamentals".to_string(),
            skill: skills::dsa_fundamentals::sliding_window_detector::TOOL_NAME.to_string(),
            route: "/api/v1/dsa_fundamentals/sliding_window_detector".to_string(),
            description: skills::dsa_fundamentals::sliding_window_detector::TOOL_DESCRIPTION
                .to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "dsa_fundamentals.{}",
                skills::dsa_fundamentals::space_calculator::TOOL_NAME
            ),
            category: "dsa_fundamentals".to_string(),
            skill: skills::dsa_fundamentals::space_calculator::TOOL_NAME.to_string(),
            route: "/api/v1/dsa_fundamentals/space_calculator".to_string(),
            description: skills::dsa_fundamentals::space_calculator::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "dsa_fundamentals.{}",
                skills::dsa_fundamentals::two_pointer_detector::TOOL_NAME
            ),
            category: "dsa_fundamentals".to_string(),
            skill: skills::dsa_fundamentals::two_pointer_detector::TOOL_NAME.to_string(),
            route: "/api/v1/dsa_fundamentals/two_pointer_detector".to_string(),
            description: skills::dsa_fundamentals::two_pointer_detector::TOOL_DESCRIPTION
                .to_string(),
        });
    }
    #[cfg(feature = "graphs")]
    {
        tools.push(McpTool {
            name: format!("graphs.{}", skills::graphs::alien_dictionary::TOOL_NAME),
            category: "graphs".to_string(),
            skill: skills::graphs::alien_dictionary::TOOL_NAME.to_string(),
            route: "/api/v1/graphs/alien_dictionary".to_string(),
            description: skills::graphs::alien_dictionary::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!("graphs.{}", skills::graphs::bellman_ford::TOOL_NAME),
            category: "graphs".to_string(),
            skill: skills::graphs::bellman_ford::TOOL_NAME.to_string(),
            route: "/api/v1/graphs/bellman_ford".to_string(),
            description: skills::graphs::bellman_ford::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!("graphs.{}", skills::graphs::bfs_generator::TOOL_NAME),
            category: "graphs".to_string(),
            skill: skills::graphs::bfs_generator::TOOL_NAME.to_string(),
            route: "/api/v1/graphs/bfs_generator".to_string(),
            description: skills::graphs::bfs_generator::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!("graphs.{}", skills::graphs::clone_graph::TOOL_NAME),
            category: "graphs".to_string(),
            skill: skills::graphs::clone_graph::TOOL_NAME.to_string(),
            route: "/api/v1/graphs/clone_graph".to_string(),
            description: skills::graphs::clone_graph::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!("graphs.{}", skills::graphs::course_schedule::TOOL_NAME),
            category: "graphs".to_string(),
            skill: skills::graphs::course_schedule::TOOL_NAME.to_string(),
            route: "/api/v1/graphs/course_schedule".to_string(),
            description: skills::graphs::course_schedule::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!("graphs.{}", skills::graphs::dfs_generator::TOOL_NAME),
            category: "graphs".to_string(),
            skill: skills::graphs::dfs_generator::TOOL_NAME.to_string(),
            route: "/api/v1/graphs/dfs_generator".to_string(),
            description: skills::graphs::dfs_generator::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!("graphs.{}", skills::graphs::dijkstra::TOOL_NAME),
            category: "graphs".to_string(),
            skill: skills::graphs::dijkstra::TOOL_NAME.to_string(),
            route: "/api/v1/graphs/dijkstra".to_string(),
            description: skills::graphs::dijkstra::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!("graphs.{}", skills::graphs::island_counter::TOOL_NAME),
            category: "graphs".to_string(),
            skill: skills::graphs::island_counter::TOOL_NAME.to_string(),
            route: "/api/v1/graphs/island_counter".to_string(),
            description: skills::graphs::island_counter::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!("graphs.{}", skills::graphs::max_area_island::TOOL_NAME),
            category: "graphs".to_string(),
            skill: skills::graphs::max_area_island::TOOL_NAME.to_string(),
            route: "/api/v1/graphs/max_area_island".to_string(),
            description: skills::graphs::max_area_island::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!("graphs.{}", skills::graphs::mst_kruskal_prim::TOOL_NAME),
            category: "graphs".to_string(),
            skill: skills::graphs::mst_kruskal_prim::TOOL_NAME.to_string(),
            route: "/api/v1/graphs/mst_kruskal_prim".to_string(),
            description: skills::graphs::mst_kruskal_prim::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!("graphs.{}", skills::graphs::rotting_oranges::TOOL_NAME),
            category: "graphs".to_string(),
            skill: skills::graphs::rotting_oranges::TOOL_NAME.to_string(),
            route: "/api/v1/graphs/rotting_oranges".to_string(),
            description: skills::graphs::rotting_oranges::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!("graphs.{}", skills::graphs::topological_sort::TOOL_NAME),
            category: "graphs".to_string(),
            skill: skills::graphs::topological_sort::TOOL_NAME.to_string(),
            route: "/api/v1/graphs/topological_sort".to_string(),
            description: skills::graphs::topological_sort::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!("graphs.{}", skills::graphs::visualizer::TOOL_NAME),
            category: "graphs".to_string(),
            skill: skills::graphs::visualizer::TOOL_NAME.to_string(),
            route: "/api/v1/graphs/visualizer".to_string(),
            description: skills::graphs::visualizer::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!("graphs.{}", skills::graphs::word_ladder::TOOL_NAME),
            category: "graphs".to_string(),
            skill: skills::graphs::word_ladder::TOOL_NAME.to_string(),
            route: "/api/v1/graphs/word_ladder".to_string(),
            description: skills::graphs::word_ladder::TOOL_DESCRIPTION.to_string(),
        });
    }
    #[cfg(feature = "greedy")]
    {
        tools.push(McpTool {
            name: format!(
                "greedy_algorithms.{}",
                skills::greedy_algorithms::activity_selection::TOOL_NAME
            ),
            category: "greedy_algorithms".to_string(),
            skill: skills::greedy_algorithms::activity_selection::TOOL_NAME.to_string(),
            route: "/api/v1/greedy_algorithms/activity_selection".to_string(),
            description: skills::greedy_algorithms::activity_selection::TOOL_DESCRIPTION
                .to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "greedy_algorithms.{}",
                skills::greedy_algorithms::fractional_knapsack::TOOL_NAME
            ),
            category: "greedy_algorithms".to_string(),
            skill: skills::greedy_algorithms::fractional_knapsack::TOOL_NAME.to_string(),
            route: "/api/v1/greedy_algorithms/fractional_knapsack".to_string(),
            description: skills::greedy_algorithms::fractional_knapsack::TOOL_DESCRIPTION
                .to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "greedy_algorithms.{}",
                skills::greedy_algorithms::gas_station::TOOL_NAME
            ),
            category: "greedy_algorithms".to_string(),
            skill: skills::greedy_algorithms::gas_station::TOOL_NAME.to_string(),
            route: "/api/v1/greedy_algorithms/gas_station".to_string(),
            description: skills::greedy_algorithms::gas_station::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "greedy_algorithms.{}",
                skills::greedy_algorithms::huffman_coding::TOOL_NAME
            ),
            category: "greedy_algorithms".to_string(),
            skill: skills::greedy_algorithms::huffman_coding::TOOL_NAME.to_string(),
            route: "/api/v1/greedy_algorithms/huffman_coding".to_string(),
            description: skills::greedy_algorithms::huffman_coding::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "greedy_algorithms.{}",
                skills::greedy_algorithms::jump_game::TOOL_NAME
            ),
            category: "greedy_algorithms".to_string(),
            skill: skills::greedy_algorithms::jump_game::TOOL_NAME.to_string(),
            route: "/api/v1/greedy_algorithms/jump_game".to_string(),
            description: skills::greedy_algorithms::jump_game::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "greedy_algorithms.{}",
                skills::greedy_algorithms::queue_reconstruct::TOOL_NAME
            ),
            category: "greedy_algorithms".to_string(),
            skill: skills::greedy_algorithms::queue_reconstruct::TOOL_NAME.to_string(),
            route: "/api/v1/greedy_algorithms/queue_reconstruct".to_string(),
            description: skills::greedy_algorithms::queue_reconstruct::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "greedy_algorithms.{}",
                skills::greedy_algorithms::task_scheduler::TOOL_NAME
            ),
            category: "greedy_algorithms".to_string(),
            skill: skills::greedy_algorithms::task_scheduler::TOOL_NAME.to_string(),
            route: "/api/v1/greedy_algorithms/task_scheduler".to_string(),
            description: skills::greedy_algorithms::task_scheduler::TOOL_DESCRIPTION.to_string(),
        });
    }
    #[cfg(feature = "linked_lists")]
    {
        tools.push(McpTool {
            name: format!(
                "linked_lists.{}",
                skills::linked_lists::add_two_numbers::TOOL_NAME
            ),
            category: "linked_lists".to_string(),
            skill: skills::linked_lists::add_two_numbers::TOOL_NAME.to_string(),
            route: "/api/v1/linked_lists/add_two_numbers".to_string(),
            description: skills::linked_lists::add_two_numbers::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "linked_lists.{}",
                skills::linked_lists::cycle_detection::TOOL_NAME
            ),
            category: "linked_lists".to_string(),
            skill: skills::linked_lists::cycle_detection::TOOL_NAME.to_string(),
            route: "/api/v1/linked_lists/cycle_detection".to_string(),
            description: skills::linked_lists::cycle_detection::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "linked_lists.{}",
                skills::linked_lists::flatten_multilevel::TOOL_NAME
            ),
            category: "linked_lists".to_string(),
            skill: skills::linked_lists::flatten_multilevel::TOOL_NAME.to_string(),
            route: "/api/v1/linked_lists/flatten_multilevel".to_string(),
            description: skills::linked_lists::flatten_multilevel::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "linked_lists.{}",
                skills::linked_lists::intersection_finder::TOOL_NAME
            ),
            category: "linked_lists".to_string(),
            skill: skills::linked_lists::intersection_finder::TOOL_NAME.to_string(),
            route: "/api/v1/linked_lists/intersection_finder".to_string(),
            description: skills::linked_lists::intersection_finder::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "linked_lists.{}",
                skills::linked_lists::merge_sorted::TOOL_NAME
            ),
            category: "linked_lists".to_string(),
            skill: skills::linked_lists::merge_sorted::TOOL_NAME.to_string(),
            route: "/api/v1/linked_lists/merge_sorted".to_string(),
            description: skills::linked_lists::merge_sorted::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "linked_lists.{}",
                skills::linked_lists::partition_list::TOOL_NAME
            ),
            category: "linked_lists".to_string(),
            skill: skills::linked_lists::partition_list::TOOL_NAME.to_string(),
            route: "/api/v1/linked_lists/partition_list".to_string(),
            description: skills::linked_lists::partition_list::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "linked_lists.{}",
                skills::linked_lists::random_pointer_copy::TOOL_NAME
            ),
            category: "linked_lists".to_string(),
            skill: skills::linked_lists::random_pointer_copy::TOOL_NAME.to_string(),
            route: "/api/v1/linked_lists/random_pointer_copy".to_string(),
            description: skills::linked_lists::random_pointer_copy::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "linked_lists.{}",
                skills::linked_lists::remove_nth_node::TOOL_NAME
            ),
            category: "linked_lists".to_string(),
            skill: skills::linked_lists::remove_nth_node::TOOL_NAME.to_string(),
            route: "/api/v1/linked_lists/remove_nth_node".to_string(),
            description: skills::linked_lists::remove_nth_node::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "linked_lists.{}",
                skills::linked_lists::reorder_list::TOOL_NAME
            ),
            category: "linked_lists".to_string(),
            skill: skills::linked_lists::reorder_list::TOOL_NAME.to_string(),
            route: "/api/v1/linked_lists/reorder_list".to_string(),
            description: skills::linked_lists::reorder_list::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "linked_lists.{}",
                skills::linked_lists::reverse_list::TOOL_NAME
            ),
            category: "linked_lists".to_string(),
            skill: skills::linked_lists::reverse_list::TOOL_NAME.to_string(),
            route: "/api/v1/linked_lists/reverse_list".to_string(),
            description: skills::linked_lists::reverse_list::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "linked_lists.{}",
                skills::linked_lists::rotate_list::TOOL_NAME
            ),
            category: "linked_lists".to_string(),
            skill: skills::linked_lists::rotate_list::TOOL_NAME.to_string(),
            route: "/api/v1/linked_lists/rotate_list".to_string(),
            description: skills::linked_lists::rotate_list::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "linked_lists.{}",
                skills::linked_lists::visualizer::TOOL_NAME
            ),
            category: "linked_lists".to_string(),
            skill: skills::linked_lists::visualizer::TOOL_NAME.to_string(),
            route: "/api/v1/linked_lists/visualizer".to_string(),
            description: skills::linked_lists::visualizer::TOOL_DESCRIPTION.to_string(),
        });
    }
    #[cfg(feature = "sorting")]
    {
        tools.push(McpTool {
            name: format!(
                "sorting_searching.{}",
                skills::sorting_searching::binary_search_template::TOOL_NAME
            ),
            category: "sorting_searching".to_string(),
            skill: skills::sorting_searching::binary_search_template::TOOL_NAME.to_string(),
            route: "/api/v1/sorting_searching/binary_search_template".to_string(),
            description: skills::sorting_searching::binary_search_template::TOOL_DESCRIPTION
                .to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "sorting_searching.{}",
                skills::sorting_searching::counting_sort::TOOL_NAME
            ),
            category: "sorting_searching".to_string(),
            skill: skills::sorting_searching::counting_sort::TOOL_NAME.to_string(),
            route: "/api/v1/sorting_searching/counting_sort".to_string(),
            description: skills::sorting_searching::counting_sort::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "sorting_searching.{}",
                skills::sorting_searching::matrix_search::TOOL_NAME
            ),
            category: "sorting_searching".to_string(),
            skill: skills::sorting_searching::matrix_search::TOOL_NAME.to_string(),
            route: "/api/v1/sorting_searching/matrix_search".to_string(),
            description: skills::sorting_searching::matrix_search::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "sorting_searching.{}",
                skills::sorting_searching::merge_sort::TOOL_NAME
            ),
            category: "sorting_searching".to_string(),
            skill: skills::sorting_searching::merge_sort::TOOL_NAME.to_string(),
            route: "/api/v1/sorting_searching/merge_sort".to_string(),
            description: skills::sorting_searching::merge_sort::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "sorting_searching.{}",
                skills::sorting_searching::peak_finder::TOOL_NAME
            ),
            category: "sorting_searching".to_string(),
            skill: skills::sorting_searching::peak_finder::TOOL_NAME.to_string(),
            route: "/api/v1/sorting_searching/peak_finder".to_string(),
            description: skills::sorting_searching::peak_finder::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "sorting_searching.{}",
                skills::sorting_searching::quick_sort::TOOL_NAME
            ),
            category: "sorting_searching".to_string(),
            skill: skills::sorting_searching::quick_sort::TOOL_NAME.to_string(),
            route: "/api/v1/sorting_searching/quick_sort".to_string(),
            description: skills::sorting_searching::quick_sort::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "sorting_searching.{}",
                skills::sorting_searching::rotated_search::TOOL_NAME
            ),
            category: "sorting_searching".to_string(),
            skill: skills::sorting_searching::rotated_search::TOOL_NAME.to_string(),
            route: "/api/v1/sorting_searching/rotated_search".to_string(),
            description: skills::sorting_searching::rotated_search::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "sorting_searching.{}",
                skills::sorting_searching::visualizer::TOOL_NAME
            ),
            category: "sorting_searching".to_string(),
            skill: skills::sorting_searching::visualizer::TOOL_NAME.to_string(),
            route: "/api/v1/sorting_searching/visualizer".to_string(),
            description: skills::sorting_searching::visualizer::TOOL_DESCRIPTION.to_string(),
        });
    }
    #[cfg(feature = "stacks")]
    {
        tools.push(McpTool {
            name: format!(
                "stacks_queues.{}",
                skills::stacks_queues::asteroid_collision::TOOL_NAME
            ),
            category: "stacks_queues".to_string(),
            skill: skills::stacks_queues::asteroid_collision::TOOL_NAME.to_string(),
            route: "/api/v1/stacks_queues/asteroid_collision".to_string(),
            description: skills::stacks_queues::asteroid_collision::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "stacks_queues.{}",
                skills::stacks_queues::daily_temperatures::TOOL_NAME
            ),
            category: "stacks_queues".to_string(),
            skill: skills::stacks_queues::daily_temperatures::TOOL_NAME.to_string(),
            route: "/api/v1/stacks_queues/daily_temperatures".to_string(),
            description: skills::stacks_queues::daily_temperatures::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "stacks_queues.{}",
                skills::stacks_queues::histogram_rectangle::TOOL_NAME
            ),
            category: "stacks_queues".to_string(),
            skill: skills::stacks_queues::histogram_rectangle::TOOL_NAME.to_string(),
            route: "/api/v1/stacks_queues/histogram_rectangle".to_string(),
            description: skills::stacks_queues::histogram_rectangle::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "stacks_queues.{}",
                skills::stacks_queues::min_stack::TOOL_NAME
            ),
            category: "stacks_queues".to_string(),
            skill: skills::stacks_queues::min_stack::TOOL_NAME.to_string(),
            route: "/api/v1/stacks_queues/min_stack".to_string(),
            description: skills::stacks_queues::min_stack::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "stacks_queues.{}",
                skills::stacks_queues::next_greater::TOOL_NAME
            ),
            category: "stacks_queues".to_string(),
            skill: skills::stacks_queues::next_greater::TOOL_NAME.to_string(),
            route: "/api/v1/stacks_queues/next_greater".to_string(),
            description: skills::stacks_queues::next_greater::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "stacks_queues.{}",
                skills::stacks_queues::queue_via_stacks::TOOL_NAME
            ),
            category: "stacks_queues".to_string(),
            skill: skills::stacks_queues::queue_via_stacks::TOOL_NAME.to_string(),
            route: "/api/v1/stacks_queues/queue_via_stacks".to_string(),
            description: skills::stacks_queues::queue_via_stacks::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "stacks_queues.{}",
                skills::stacks_queues::reverse_polish::TOOL_NAME
            ),
            category: "stacks_queues".to_string(),
            skill: skills::stacks_queues::reverse_polish::TOOL_NAME.to_string(),
            route: "/api/v1/stacks_queues/reverse_polish".to_string(),
            description: skills::stacks_queues::reverse_polish::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "stacks_queues.{}",
                skills::stacks_queues::stack_via_queues::TOOL_NAME
            ),
            category: "stacks_queues".to_string(),
            skill: skills::stacks_queues::stack_via_queues::TOOL_NAME.to_string(),
            route: "/api/v1/stacks_queues/stack_via_queues".to_string(),
            description: skills::stacks_queues::stack_via_queues::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "stacks_queues.{}",
                skills::stacks_queues::valid_parentheses::TOOL_NAME
            ),
            category: "stacks_queues".to_string(),
            skill: skills::stacks_queues::valid_parentheses::TOOL_NAME.to_string(),
            route: "/api/v1/stacks_queues/valid_parentheses".to_string(),
            description: skills::stacks_queues::valid_parentheses::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "stacks_queues.{}",
                skills::stacks_queues::visualizer::TOOL_NAME
            ),
            category: "stacks_queues".to_string(),
            skill: skills::stacks_queues::visualizer::TOOL_NAME.to_string(),
            route: "/api/v1/stacks_queues/visualizer".to_string(),
            description: skills::stacks_queues::visualizer::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "stacks_queues.{}",
                skills::stacks_queues::window_maximum::TOOL_NAME
            ),
            category: "stacks_queues".to_string(),
            skill: skills::stacks_queues::window_maximum::TOOL_NAME.to_string(),
            route: "/api/v1/stacks_queues/window_maximum".to_string(),
            description: skills::stacks_queues::window_maximum::TOOL_DESCRIPTION.to_string(),
        });
    }
    #[cfg(feature = "trees")]
    {
        tools.push(McpTool {
            name: format!(
                "trees_binary.{}",
                skills::trees_binary::balance_checker::TOOL_NAME
            ),
            category: "trees_binary".to_string(),
            skill: skills::trees_binary::balance_checker::TOOL_NAME.to_string(),
            route: "/api/v1/trees_binary/balance_checker".to_string(),
            description: skills::trees_binary::balance_checker::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "trees_binary.{}",
                skills::trees_binary::bst_validator::TOOL_NAME
            ),
            category: "trees_binary".to_string(),
            skill: skills::trees_binary::bst_validator::TOOL_NAME.to_string(),
            route: "/api/v1/trees_binary/bst_validator".to_string(),
            description: skills::trees_binary::bst_validator::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "trees_binary.{}",
                skills::trees_binary::construct_from_traversal::TOOL_NAME
            ),
            category: "trees_binary".to_string(),
            skill: skills::trees_binary::construct_from_traversal::TOOL_NAME.to_string(),
            route: "/api/v1/trees_binary/construct_from_traversal".to_string(),
            description: skills::trees_binary::construct_from_traversal::TOOL_DESCRIPTION
                .to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "trees_binary.{}",
                skills::trees_binary::height_calc::TOOL_NAME
            ),
            category: "trees_binary".to_string(),
            skill: skills::trees_binary::height_calc::TOOL_NAME.to_string(),
            route: "/api/v1/trees_binary/height_calc".to_string(),
            description: skills::trees_binary::height_calc::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "trees_binary.{}",
                skills::trees_binary::invert_tree::TOOL_NAME
            ),
            category: "trees_binary".to_string(),
            skill: skills::trees_binary::invert_tree::TOOL_NAME.to_string(),
            route: "/api/v1/trees_binary/invert_tree".to_string(),
            description: skills::trees_binary::invert_tree::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "trees_binary.{}",
                skills::trees_binary::lca_finder::TOOL_NAME
            ),
            category: "trees_binary".to_string(),
            skill: skills::trees_binary::lca_finder::TOOL_NAME.to_string(),
            route: "/api/v1/trees_binary/lca_finder".to_string(),
            description: skills::trees_binary::lca_finder::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "trees_binary.{}",
                skills::trees_binary::max_path_sum::TOOL_NAME
            ),
            category: "trees_binary".to_string(),
            skill: skills::trees_binary::max_path_sum::TOOL_NAME.to_string(),
            route: "/api/v1/trees_binary/max_path_sum".to_string(),
            description: skills::trees_binary::max_path_sum::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!("trees_binary.{}", skills::trees_binary::path_sum::TOOL_NAME),
            category: "trees_binary".to_string(),
            skill: skills::trees_binary::path_sum::TOOL_NAME.to_string(),
            route: "/api/v1/trees_binary/path_sum".to_string(),
            description: skills::trees_binary::path_sum::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "trees_binary.{}",
                skills::trees_binary::same_tree::TOOL_NAME
            ),
            category: "trees_binary".to_string(),
            skill: skills::trees_binary::same_tree::TOOL_NAME.to_string(),
            route: "/api/v1/trees_binary/same_tree".to_string(),
            description: skills::trees_binary::same_tree::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "trees_binary.{}",
                skills::trees_binary::serialization::TOOL_NAME
            ),
            category: "trees_binary".to_string(),
            skill: skills::trees_binary::serialization::TOOL_NAME.to_string(),
            route: "/api/v1/trees_binary/serialization".to_string(),
            description: skills::trees_binary::serialization::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "trees_binary.{}",
                skills::trees_binary::subtree_validator::TOOL_NAME
            ),
            category: "trees_binary".to_string(),
            skill: skills::trees_binary::subtree_validator::TOOL_NAME.to_string(),
            route: "/api/v1/trees_binary/subtree_validator".to_string(),
            description: skills::trees_binary::subtree_validator::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "trees_binary.{}",
                skills::trees_binary::symmetric_checker::TOOL_NAME
            ),
            category: "trees_binary".to_string(),
            skill: skills::trees_binary::symmetric_checker::TOOL_NAME.to_string(),
            route: "/api/v1/trees_binary/symmetric_checker".to_string(),
            description: skills::trees_binary::symmetric_checker::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "trees_binary.{}",
                skills::trees_binary::traversals::TOOL_NAME
            ),
            category: "trees_binary".to_string(),
            skill: skills::trees_binary::traversals::TOOL_NAME.to_string(),
            route: "/api/v1/trees_binary/traversals".to_string(),
            description: skills::trees_binary::traversals::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "trees_binary.{}",
                skills::trees_binary::visualizer::TOOL_NAME
            ),
            category: "trees_binary".to_string(),
            skill: skills::trees_binary::visualizer::TOOL_NAME.to_string(),
            route: "/api/v1/trees_binary/visualizer".to_string(),
            description: skills::trees_binary::visualizer::TOOL_DESCRIPTION.to_string(),
        });
    }
    #[cfg(feature = "trees_adv")]
    {
        tools.push(McpTool {
            name: format!(
                "trees_advanced.{}",
                skills::trees_advanced::avl_tree::TOOL_NAME
            ),
            category: "trees_advanced".to_string(),
            skill: skills::trees_advanced::avl_tree::TOOL_NAME.to_string(),
            route: "/api/v1/trees_advanced/avl_tree".to_string(),
            description: skills::trees_advanced::avl_tree::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "trees_advanced.{}",
                skills::trees_advanced::b_tree_index::TOOL_NAME
            ),
            category: "trees_advanced".to_string(),
            skill: skills::trees_advanced::b_tree_index::TOOL_NAME.to_string(),
            route: "/api/v1/trees_advanced/b_tree_index".to_string(),
            description: skills::trees_advanced::b_tree_index::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "trees_advanced.{}",
                skills::trees_advanced::fenwick_tree::TOOL_NAME
            ),
            category: "trees_advanced".to_string(),
            skill: skills::trees_advanced::fenwick_tree::TOOL_NAME.to_string(),
            route: "/api/v1/trees_advanced/fenwick_tree".to_string(),
            description: skills::trees_advanced::fenwick_tree::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "trees_advanced.{}",
                skills::trees_advanced::heap_priority_queue::TOOL_NAME
            ),
            category: "trees_advanced".to_string(),
            skill: skills::trees_advanced::heap_priority_queue::TOOL_NAME.to_string(),
            route: "/api/v1/trees_advanced/heap_priority_queue".to_string(),
            description: skills::trees_advanced::heap_priority_queue::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "trees_advanced.{}",
                skills::trees_advanced::median_stream::TOOL_NAME
            ),
            category: "trees_advanced".to_string(),
            skill: skills::trees_advanced::median_stream::TOOL_NAME.to_string(),
            route: "/api/v1/trees_advanced/median_stream".to_string(),
            description: skills::trees_advanced::median_stream::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "trees_advanced.{}",
                skills::trees_advanced::red_black_tree::TOOL_NAME
            ),
            category: "trees_advanced".to_string(),
            skill: skills::trees_advanced::red_black_tree::TOOL_NAME.to_string(),
            route: "/api/v1/trees_advanced/red_black_tree".to_string(),
            description: skills::trees_advanced::red_black_tree::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "trees_advanced.{}",
                skills::trees_advanced::segment_tree_builder::TOOL_NAME
            ),
            category: "trees_advanced".to_string(),
            skill: skills::trees_advanced::segment_tree_builder::TOOL_NAME.to_string(),
            route: "/api/v1/trees_advanced/segment_tree_builder".to_string(),
            description: skills::trees_advanced::segment_tree_builder::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "trees_advanced.{}",
                skills::trees_advanced::segment_tree_query::TOOL_NAME
            ),
            category: "trees_advanced".to_string(),
            skill: skills::trees_advanced::segment_tree_query::TOOL_NAME.to_string(),
            route: "/api/v1/trees_advanced/segment_tree_query".to_string(),
            description: skills::trees_advanced::segment_tree_query::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "trees_advanced.{}",
                skills::trees_advanced::top_k_elements::TOOL_NAME
            ),
            category: "trees_advanced".to_string(),
            skill: skills::trees_advanced::top_k_elements::TOOL_NAME.to_string(),
            route: "/api/v1/trees_advanced/top_k_elements".to_string(),
            description: skills::trees_advanced::top_k_elements::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "trees_advanced.{}",
                skills::trees_advanced::trie_impl::TOOL_NAME
            ),
            category: "trees_advanced".to_string(),
            skill: skills::trees_advanced::trie_impl::TOOL_NAME.to_string(),
            route: "/api/v1/trees_advanced/trie_impl".to_string(),
            description: skills::trees_advanced::trie_impl::TOOL_DESCRIPTION.to_string(),
        });
        tools.push(McpTool {
            name: format!(
                "trees_advanced.{}",
                skills::trees_advanced::trie_visualizer::TOOL_NAME
            ),
            category: "trees_advanced".to_string(),
            skill: skills::trees_advanced::trie_visualizer::TOOL_NAME.to_string(),
            route: "/api/v1/trees_advanced/trie_visualizer".to_string(),
            description: skills::trees_advanced::trie_visualizer::TOOL_DESCRIPTION.to_string(),
        });
    }
    tools
}
