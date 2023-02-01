#![warn(
    absolute_paths_not_starting_with_crate,
    explicit_outlives_requirements,
    let_underscore_drop,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_docs,
    noop_method_call,
    pointer_structural_match,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    unused_tuple_struct_fields,
    variant_size_differences,
    clippy::alloc_instead_of_core,
    clippy::allow_attributes_without_reason,
    clippy::as_ptr_cast_mut,
    clippy::branches_sharing_code,
    clippy::cargo_common_metadata,
    clippy::clone_on_ref_ptr,
    clippy::cognitive_complexity,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::decimal_literal_representation,
    clippy::deref_by_slicing,
    clippy::derive_partial_eq_without_eq,
    clippy::empty_drop,
    clippy::empty_line_after_outer_attr,
    clippy::empty_structs_with_brackets,
    clippy::equatable_if_let,
    clippy::fallible_impl_from,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::format_push_string,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::imprecise_flops,
    clippy::iter_on_empty_collections,
    clippy::iter_on_single_items,
    clippy::iter_with_drain,
    clippy::large_include_file,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::manual_clamp,
    clippy::map_err_ignore,
    clippy::mixed_read_write_in_expression,
    clippy::multiple_inherent_impl,
    clippy::mutex_atomic,
    clippy::mutex_integer,
    clippy::needless_collect,
    clippy::negative_feature_names,
    clippy::non_send_fields_in_send_ty,
    clippy::nonstandard_macro_braces,
    clippy::option_if_let_else,
    clippy::or_fun_call,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::partial_pub_fields,
    clippy::path_buf_push_overwrite,
    clippy::pedantic,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::redundant_feature_names,
    clippy::redundant_pub_crate,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::self_named_module_files,
    clippy::significant_drop_in_scrutinee,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::suboptimal_flops,
    clippy::suspicious_operation_groupings,
    clippy::todo,
    clippy::trailing_empty_array,
    clippy::trait_duplication_in_bounds,
    clippy::transmute_undefined_repr,
    clippy::trivial_regex,
    clippy::try_err,
    clippy::type_repetition_in_bounds,
    clippy::undocumented_unsafe_blocks,
    clippy::unimplemented,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unused_peekable,
    clippy::unused_rounding,
    clippy::use_debug,
    clippy::use_self,
    clippy::useless_let_if_seq,
    clippy::verbose_file_reads,
    clippy::wildcard_dependencies
)]
#![allow(
    missing_docs,
    trivial_casts,
    clippy::cargo_common_metadata,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::decimal_literal_representation,
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::same_name_method,
    clippy::wildcard_dependencies
)]

pub(crate) mod data_structures;
pub mod problem_0001_two_sum;
pub mod problem_0002_add_two_numbers;
pub mod problem_0003_longest_substring_without_repeating_characters;
pub mod problem_0004_median_of_two_sorted_arrays;
pub mod problem_0005_longest_palindromic_substring;
pub mod problem_0006_zigzag_conversion;
pub mod problem_0007_reverse_integer;
pub mod problem_0008_string_to_integer_atoi;
pub mod problem_0009_palindrome_number;
pub mod problem_0010_regular_expression_matching;
pub mod problem_0011_container_with_most_water;
pub mod problem_0012_integer_to_roman;
pub mod problem_0013_roman_to_integer;
pub mod problem_0014_longest_common_prefix;
pub mod problem_0015_3sum;
pub mod problem_0016_3sum_closest;
pub mod problem_0017_letter_combinations_of_a_phone_number;
pub mod problem_0018_4sum;
pub mod problem_0019_remove_nth_node_from_end_of_list;
pub mod problem_0020_valid_parentheses;
pub mod problem_0021_merge_two_sorted_lists;
pub mod problem_0022_generate_parentheses;
pub mod problem_0023_merge_k_sorted_lists;
pub mod problem_0024_swap_nodes_in_pairs;
pub mod problem_0025_reverse_nodes_in_k_group;
pub mod problem_0026_remove_duplicates_from_sorted_array;
pub mod problem_0027_remove_element;
pub mod problem_0028_find_the_index_of_the_first_occurrence_in_a_string;
pub mod problem_0029_divide_two_integers;
pub mod problem_0030_substring_with_concatenation_of_all_words;
pub mod problem_0031_next_permutation;
pub mod problem_0032_longest_valid_parentheses;
pub mod problem_0033_search_in_rotated_sorted_array;
pub mod problem_0034_find_first_and_last_position_of_element_in_sorted_array;
pub mod problem_0035_search_insert_position;
pub mod problem_0036_valid_sudoku;
pub mod problem_0037_sudoku_solver;
pub mod problem_0038_count_and_say;
pub mod problem_0039_combination_sum;
pub mod problem_0040_combination_sum_ii;
pub mod problem_0041_first_missing_positive;
pub mod problem_0042_trapping_rain_water;
pub mod problem_0043_multiply_strings;
pub mod problem_0044_wildcard_matching;
pub mod problem_0045_jump_game_ii;
pub mod problem_0046_permutations;
pub mod problem_0047_permutations_ii;
pub mod problem_0048_rotate_image;
pub mod problem_0049_group_anagrams;
pub mod problem_0050_powx_n;
pub mod problem_0051_n_queens;
pub mod problem_0052_n_queens_ii;
pub mod problem_0053_maximum_subarray;
pub mod problem_0054_spiral_matrix;
pub mod problem_0055_jump_game;
pub mod problem_0056_merge_intervals;
pub mod problem_0057_insert_interval;
pub mod problem_0058_length_of_last_word;
pub mod problem_0059_spiral_matrix_ii;
pub mod problem_0060_permutation_sequence;
pub mod problem_0061_rotate_list;
pub mod problem_0062_unique_paths;
pub mod problem_0063_unique_paths_ii;
pub mod problem_0064_minimum_path_sum;
pub mod problem_0065_valid_number;
pub mod problem_0066_plus_one;
pub mod problem_0067_add_binary;
pub mod problem_0068_text_justification;
pub mod problem_0069_sqrtx;
pub mod problem_0070_climbing_stairs;
pub mod problem_0071_simplify_path;
pub mod problem_0072_edit_distance;
pub mod problem_0073_set_matrix_zeroes;
pub mod problem_0074_search_a_2d_matrix;
pub mod problem_0075_sort_colors;
pub mod problem_0076_minimum_window_substring;
pub mod problem_0077_combinations;
pub mod problem_0078_subsets;
pub mod problem_0079_word_search;
pub mod problem_0080_remove_duplicates_from_sorted_array_ii;
pub mod problem_0081_search_in_rotated_sorted_array_ii;
pub mod problem_0082_remove_duplicates_from_sorted_list_ii;
pub mod problem_0083_remove_duplicates_from_sorted_list;
pub mod problem_0084_largest_rectangle_in_histogram;
pub mod problem_0085_maximal_rectangle;
pub mod problem_0086_partition_list;
pub mod problem_0087_scramble_string;
pub mod problem_0088_merge_sorted_array;
pub mod problem_0089_gray_code;
pub mod problem_0090_subsets_ii;
pub mod problem_0091_decode_ways;
pub mod problem_0092_reverse_linked_list_ii;
pub mod problem_0093_restore_ip_addresses;
pub mod problem_0094_binary_tree_inorder_traversal;
pub mod problem_0095_unique_binary_search_trees_ii;
pub mod problem_0096_unique_binary_search_trees;
pub mod problem_0097_interleaving_string;
pub mod problem_0098_validate_binary_search_tree;
pub mod problem_0099_recover_binary_search_tree;
pub mod problem_0100_same_tree;
pub mod problem_0101_symmetric_tree;
pub mod problem_0102_binary_tree_level_order_traversal;
pub mod problem_0103_binary_tree_zigzag_level_order_traversal;
pub mod problem_0104_maximum_depth_of_binary_tree;
pub mod problem_0105_construct_binary_tree_from_preorder_and_inorder_traversal;
pub mod problem_0106_construct_binary_tree_from_inorder_and_postorder_traversal;
pub mod problem_0107_binary_tree_level_order_traversal_ii;
pub mod problem_0108_convert_sorted_array_to_binary_search_tree;
pub mod problem_0109_convert_sorted_list_to_binary_search_tree;
pub mod problem_0110_balanced_binary_tree;
pub mod problem_0111_minimum_depth_of_binary_tree;
pub mod problem_0112_path_sum;
pub mod problem_0113_path_sum_ii;
pub mod problem_0114_flatten_binary_tree_to_linked_list;
pub mod problem_0115_distinct_subsequences;
pub mod problem_0118_pascals_triangle;
pub mod problem_0119_pascals_triangle_ii;
pub mod problem_0120_triangle;
pub mod problem_0121_best_time_to_buy_and_sell_stock;
pub mod problem_0122_best_time_to_buy_and_sell_stock_ii;
pub mod problem_0123_best_time_to_buy_and_sell_stock_iii;
pub mod problem_0124_binary_tree_maximum_path_sum;
pub mod problem_0125_valid_palindrome;
pub mod problem_0126_word_ladder_ii;
pub mod problem_0127_word_ladder;
pub mod problem_0128_longest_consecutive_sequence;
pub mod problem_0129_sum_root_to_leaf_numbers;
pub mod problem_0130_surrounded_regions;
pub mod problem_0131_palindrome_partitioning;
pub mod problem_0132_palindrome_partitioning_ii;
pub mod problem_0134_gas_station;
pub mod problem_0135_candy;
pub mod problem_0136_single_number;
pub mod problem_0137_single_number_ii;
pub mod problem_0139_word_break;
pub mod problem_0140_word_break_ii;
pub mod problem_0143_reorder_list;
pub mod problem_0144_binary_tree_preorder_traversal;
pub mod problem_0145_binary_tree_postorder_traversal;
pub mod problem_0146_lru_cache;
pub mod problem_0147_insertion_sort_list;
pub mod problem_0148_sort_list;
pub mod problem_0149_max_points_on_a_line;
pub mod problem_0150_evaluate_reverse_polish_notation;
pub mod problem_0151_reverse_words_in_a_string;
pub mod problem_0152_maximum_product_subarray;
pub mod problem_0153_find_minimum_in_rotated_sorted_array;
pub mod problem_0154_find_minimum_in_rotated_sorted_array_ii;
pub mod problem_0155_min_stack;
pub mod problem_0162_find_peak_element;
pub mod problem_0164_maximum_gap;
pub mod problem_0165_compare_version_numbers;
pub mod problem_0166_fraction_to_recurring_decimal;
pub mod problem_0167_two_sum_ii_input_array_is_sorted;
pub mod problem_0168_excel_sheet_column_title;
pub mod problem_0169_majority_element;
pub mod problem_0171_excel_sheet_column_number;
pub mod problem_0172_factorial_trailing_zeroes;
pub mod problem_0173_binary_search_tree_iterator;
pub mod problem_0174_dungeon_game;
pub mod problem_0179_largest_number;
pub mod problem_0187_repeated_dna_sequences;
pub mod problem_0188_best_time_to_buy_and_sell_stock_iv;
pub mod problem_0189_rotate_array;
pub mod problem_0190_reverse_bits;
pub mod problem_0191_number_of_1_bits;
pub mod problem_0198_house_robber;
pub mod problem_0199_binary_tree_right_side_view;
pub mod problem_0200_number_of_islands;
pub mod problem_0201_bitwise_and_of_numbers_range;
pub mod problem_0202_happy_number;
pub mod problem_0203_remove_linked_list_elements;
pub mod problem_0204_count_primes;
pub mod problem_0205_isomorphic_strings;
pub mod problem_0206_reverse_linked_list;
pub mod problem_0207_course_schedule;
pub mod problem_0208_implement_trie_prefix_tree;
pub mod problem_0209_minimum_size_subarray_sum;
pub mod problem_0210_course_schedule_ii;
pub mod problem_0211_design_add_and_search_words_data_structure;
pub mod problem_0212_word_search_ii;
pub mod problem_0213_house_robber_ii;
pub mod problem_0214_shortest_palindrome;
pub mod problem_0215_kth_largest_element_in_an_array;
pub mod problem_0216_combination_sum_iii;
pub mod problem_0217_contains_duplicate;
pub mod problem_0218_the_skyline_problem;
pub mod problem_0219_contains_duplicate_ii;
pub mod problem_0220_contains_duplicate_iii;
pub mod problem_0221_maximal_square;
pub mod problem_0222_count_complete_tree_nodes;
pub mod problem_0223_rectangle_area;
pub mod problem_0224_basic_calculator;
pub mod problem_0225_implement_stack_using_queues;
pub mod problem_0226_invert_binary_tree;
pub mod problem_0227_basic_calculator_ii;
pub mod problem_0228_summary_ranges;
pub mod problem_0229_majority_element_ii;
pub mod problem_0230_kth_smallest_element_in_a_bst;
pub mod problem_0231_power_of_two;
pub mod problem_0232_implement_queue_using_stacks;
pub mod problem_0233_number_of_digit_one;
pub mod problem_0234_palindrome_linked_list;
pub mod problem_0235_lowest_common_ancestor_of_a_binary_search_tree;
pub mod problem_0236_lowest_common_ancestor_of_a_binary_tree;
pub mod problem_0238_product_of_array_except_self;
pub mod problem_0239_sliding_window_maximum;
pub mod problem_0240_search_a_2d_matrix_ii;
pub mod problem_0241_different_ways_to_add_parentheses;
pub mod problem_0242_valid_anagram;
pub mod problem_0257_binary_tree_paths;
pub mod problem_0258_add_digits;
pub mod problem_0260_single_number_iii;
pub mod problem_0263_ugly_number;
pub mod problem_0264_ugly_number_ii;
pub mod problem_0268_missing_number;
pub mod problem_0273_integer_to_english_words;
pub mod problem_0274_h_index;
pub mod problem_0275_h_index_ii;
pub mod problem_0278_first_bad_version;
pub mod problem_0279_perfect_squares;
pub mod problem_0282_expression_add_operators;
pub mod problem_0283_move_zeroes;
pub mod problem_0287_find_the_duplicate_number;
pub mod problem_0289_game_of_life;
pub mod problem_0290_word_pattern;
pub mod problem_0292_nim_game;
pub mod problem_0295_find_median_from_data_stream;
pub mod problem_0297_serialize_and_deserialize_binary_tree;
pub mod problem_0299_bulls_and_cows;
pub mod problem_0300_longest_increasing_subsequence;
pub mod problem_0301_remove_invalid_parentheses;
pub mod problem_0303_range_sum_query_immutable;
pub mod problem_0304_range_sum_query_2d_immutable;
pub mod problem_0306_additive_number;
pub mod problem_0307_range_sum_query_mutable;
pub mod problem_0309_best_time_to_buy_and_sell_stock_with_cooldown;
pub mod problem_0310_minimum_height_trees;
pub mod problem_0312_burst_balloons;
pub mod problem_0313_super_ugly_number;
pub mod problem_0315_count_of_smaller_numbers_after_self;
pub mod problem_0316_remove_duplicate_letters;
pub mod problem_0318_maximum_product_of_word_lengths;
pub mod problem_0319_bulb_switcher;
pub mod problem_0321_create_maximum_number;
pub mod problem_0322_coin_change;
pub mod problem_0324_wiggle_sort_ii;
pub mod problem_0326_power_of_three;
pub mod problem_0327_count_of_range_sum;
pub mod problem_0328_odd_even_linked_list;
pub mod problem_0329_longest_increasing_path_in_a_matrix;
pub mod problem_0330_patching_array;
pub mod problem_0331_verify_preorder_serialization_of_a_binary_tree;
pub mod problem_0332_reconstruct_itinerary;
pub mod problem_0334_increasing_triplet_subsequence;
pub mod problem_0335_self_crossing;
pub mod problem_0336_palindrome_pairs;
pub mod problem_0337_house_robber_iii;
pub mod problem_0338_counting_bits;
pub mod problem_0341_flatten_nested_list_iterator;
pub mod problem_0342_power_of_four;
pub mod problem_0343_integer_break;
pub mod problem_0344_reverse_string;
pub mod problem_0345_reverse_vowels_of_a_string;
pub mod problem_0347_top_k_frequent_elements;
pub mod problem_0349_intersection_of_two_arrays;
pub mod problem_0350_intersection_of_two_arrays_ii;
pub mod problem_0352_data_stream_as_disjoint_intervals;
pub mod problem_0354_russian_doll_envelopes;
pub mod problem_0355_design_twitter;
pub mod problem_0357_count_numbers_with_unique_digits;
pub mod problem_0363_max_sum_of_rectangle_no_larger_than_k;
pub mod problem_0365_water_and_jug_problem;
pub mod problem_0367_valid_perfect_square;
pub mod problem_0368_largest_divisible_subset;
pub mod problem_0371_sum_of_two_integers;
pub mod problem_0372_super_pow;
pub mod problem_0373_find_k_pairs_with_smallest_sums;
pub mod problem_0374_guess_number_higher_or_lower;
pub mod problem_0375_guess_number_higher_or_lower_ii;
pub mod problem_0376_wiggle_subsequence;
pub mod problem_0377_combination_sum_iv;
pub mod problem_0378_kth_smallest_element_in_a_sorted_matrix;
pub mod problem_0383_ransom_note;
pub mod problem_0385_mini_parser;
pub mod problem_0386_lexicographical_numbers;
pub mod problem_0387_first_unique_character_in_a_string;
pub mod problem_0388_longest_absolute_file_path;
pub mod problem_0389_find_the_difference;
pub mod problem_0390_elimination_game;
pub mod problem_0391_perfect_rectangle;
pub mod problem_0392_is_subsequence;
pub mod problem_0393_utf_8_validation;
pub mod problem_0394_decode_string;
pub mod problem_0395_longest_substring_with_at_least_k_repeating_characters;
pub mod problem_0396_rotate_function;
pub mod problem_0397_integer_replacement;
pub mod problem_0399_evaluate_division;
pub mod problem_0400_nth_digit;
pub mod problem_0401_binary_watch;
pub mod problem_0402_remove_k_digits;
pub mod problem_0403_frog_jump;
pub mod problem_0404_sum_of_left_leaves;
pub mod problem_0405_convert_a_number_to_hexadecimal;
pub mod problem_0406_queue_reconstruction_by_height;
pub mod problem_0407_trapping_rain_water_ii;
pub mod problem_0409_longest_palindrome;
pub mod problem_0410_split_array_largest_sum;
pub mod problem_0412_fizz_buzz;
pub mod problem_0413_arithmetic_slices;
pub mod problem_0414_third_maximum_number;
pub mod problem_0415_add_strings;
pub mod problem_0416_partition_equal_subset_sum;
pub mod problem_0417_pacific_atlantic_water_flow;
pub mod problem_0419_battleships_in_a_board;
pub mod problem_0420_strong_password_checker;
pub mod problem_0421_maximum_xor_of_two_numbers_in_an_array;
pub mod problem_0423_reconstruct_original_digits_from_english;
pub mod problem_0424_longest_repeating_character_replacement;
pub mod problem_0432_all_oone_data_structure;
pub mod problem_0433_minimum_genetic_mutation;
pub mod problem_0434_number_of_segments_in_a_string;
pub mod problem_0435_non_overlapping_intervals;
pub mod problem_0436_find_right_interval;
pub mod problem_0437_path_sum_iii;
pub mod problem_0438_find_all_anagrams_in_a_string;
pub mod problem_0440_k_th_smallest_in_lexicographical_order;
pub mod problem_0441_arranging_coins;
pub mod problem_0442_find_all_duplicates_in_an_array;
pub mod problem_0443_string_compression;
pub mod problem_0445_add_two_numbers_ii;
pub mod problem_0446_arithmetic_slices_ii_subsequence;
pub mod problem_0447_number_of_boomerangs;
pub mod problem_0448_find_all_numbers_disappeared_in_an_array;
pub mod problem_0449_serialize_and_deserialize_bst;
pub mod problem_0450_delete_node_in_a_bst;
pub mod problem_0451_sort_characters_by_frequency;
pub mod problem_0452_minimum_number_of_arrows_to_burst_balloons;
pub mod problem_0453_minimum_moves_to_equal_array_elements;
pub mod problem_0454_4sum_ii;
pub mod problem_0455_assign_cookies;
pub mod problem_0456_132_pattern;
pub mod problem_0457_circular_array_loop;
pub mod problem_0458_poor_pigs;
pub mod problem_0459_repeated_substring_pattern;
pub mod problem_0460_lfu_cache;
pub mod problem_0461_hamming_distance;
pub mod problem_0462_minimum_moves_to_equal_array_elements_ii;
pub mod problem_0463_island_perimeter;
pub mod problem_0464_can_i_win;
pub mod problem_0466_count_the_repetitions;
pub mod problem_0467_unique_substrings_in_wraparound_string;
pub mod problem_0468_validate_ip_address;
pub mod problem_0472_concatenated_words;
pub mod problem_0473_matchsticks_to_square;
pub mod problem_0474_ones_and_zeroes;
pub mod problem_0475_heaters;
pub mod problem_0476_number_complement;
pub mod problem_0477_total_hamming_distance;
pub mod problem_0479_largest_palindrome_product;
pub mod problem_0480_sliding_window_median;
pub mod problem_0481_magical_string;
pub mod problem_0482_license_key_formatting;
pub mod problem_0483_smallest_good_base;
pub mod problem_0485_max_consecutive_ones;
pub mod problem_0486_predict_the_winner;
pub mod problem_0488_zuma_game;
pub mod problem_0491_increasing_subsequences;
pub mod problem_0492_construct_the_rectangle;
pub mod problem_0493_reverse_pairs;
pub mod problem_0494_target_sum;
pub mod problem_0495_teemo_attacking;
pub mod problem_0496_next_greater_element_i;
pub mod problem_0498_diagonal_traverse;
pub mod problem_0500_keyboard_row;
pub mod problem_0501_find_mode_in_binary_search_tree;
pub mod problem_0502_ipo;
pub mod problem_0503_next_greater_element_ii;
pub mod problem_0504_base_7;
pub mod problem_0506_relative_ranks;
pub mod problem_0507_perfect_number;
pub mod problem_0508_most_frequent_subtree_sum;
pub mod problem_0509_fibonacci_number;
pub mod problem_0513_find_bottom_left_tree_value;
pub mod problem_0514_freedom_trail;
pub mod problem_0515_find_largest_value_in_each_tree_row;
pub mod problem_0516_longest_palindromic_subsequence;
pub mod problem_0517_super_washing_machines;
pub mod problem_0518_coin_change_ii;
pub mod problem_0520_detect_capital;
pub mod problem_0521_longest_uncommon_subsequence_i;
pub mod problem_0522_longest_uncommon_subsequence_ii;
pub mod problem_0523_continuous_subarray_sum;
pub mod problem_0524_longest_word_in_dictionary_through_deleting;
pub mod problem_0525_contiguous_array;
pub mod problem_0526_beautiful_arrangement;
pub mod problem_0529_minesweeper;
pub mod problem_0530_minimum_absolute_difference_in_bst;
pub mod problem_0532_k_diff_pairs_in_an_array;
pub mod problem_0535_encode_and_decode_tinyurl;
pub mod problem_0537_complex_number_multiplication;
pub mod problem_0538_convert_bst_to_greater_tree;
pub mod problem_0539_minimum_time_difference;
pub mod problem_0540_single_element_in_a_sorted_array;
pub mod problem_0541_reverse_string_ii;
pub mod problem_0542_01_matrix;
pub mod problem_0543_diameter_of_binary_tree;
pub mod problem_0546_remove_boxes;
pub mod problem_0547_number_of_provinces;
pub mod problem_0551_student_attendance_record_i;
pub mod problem_0552_student_attendance_record_ii;
pub mod problem_0553_optimal_division;
pub mod problem_0554_brick_wall;
pub mod problem_0556_next_greater_element_iii;
pub mod problem_0557_reverse_words_in_a_string_iii;
pub mod problem_0560_subarray_sum_equals_k;
pub mod problem_0561_array_partition;
pub mod problem_0563_binary_tree_tilt;
pub mod problem_0564_find_the_closest_palindrome;
pub mod problem_0565_array_nesting;
pub mod problem_0566_reshape_the_matrix;
pub mod problem_0567_permutation_in_string;
pub mod problem_0572_subtree_of_another_tree;
pub mod problem_0575_distribute_candies;
pub mod problem_0576_out_of_boundary_paths;
pub mod problem_0581_shortest_unsorted_continuous_subarray;
pub mod problem_0583_delete_operation_for_two_strings;
pub mod problem_0587_erect_the_fence;
pub mod problem_0591_tag_validator;
pub mod problem_0592_fraction_addition_and_subtraction;
pub mod problem_0593_valid_square;
pub mod problem_0594_longest_harmonious_subsequence;
pub mod problem_0598_range_addition_ii;
pub mod problem_0599_minimum_index_sum_of_two_lists;
pub mod problem_0600_non_negative_integers_without_consecutive_ones;
pub mod problem_0605_can_place_flowers;
pub mod problem_0606_construct_string_from_binary_tree;
pub mod problem_0609_find_duplicate_file_in_system;
pub mod problem_0611_valid_triangle_number;
pub mod problem_0617_merge_two_binary_trees;
pub mod problem_0621_task_scheduler;
pub mod problem_0622_design_circular_queue;
pub mod problem_0623_add_one_row_to_tree;
pub mod problem_0628_maximum_product_of_three_numbers;
pub mod problem_0629_k_inverse_pairs_array;
pub mod problem_0630_course_schedule_iii;
pub mod problem_0632_smallest_range_covering_elements_from_k_lists;
pub mod problem_0633_sum_of_square_numbers;
pub mod problem_0636_exclusive_time_of_functions;
pub mod problem_0637_average_of_levels_in_binary_tree;
pub mod problem_0638_shopping_offers;
pub mod problem_0639_decode_ways_ii;
pub mod problem_0640_solve_the_equation;
pub mod problem_0641_design_circular_deque;
pub mod problem_0643_maximum_average_subarray_i;
pub mod problem_0645_set_mismatch;
pub mod problem_0646_maximum_length_of_pair_chain;
pub mod problem_0647_palindromic_substrings;
pub mod problem_0648_replace_words;
pub mod problem_0649_dota2_senate;
pub mod problem_0650_2_keys_keyboard;
pub mod problem_0652_find_duplicate_subtrees;
pub mod problem_0653_two_sum_iv_input_is_a_bst;
pub mod problem_0654_maximum_binary_tree;
pub mod problem_0655_print_binary_tree;
pub mod problem_0657_robot_return_to_origin;
pub mod problem_0658_find_k_closest_elements;
pub mod problem_0659_split_array_into_consecutive_subsequences;
pub mod problem_0661_image_smoother;
pub mod problem_0662_maximum_width_of_binary_tree;
pub mod problem_0664_strange_printer;
pub mod problem_0665_non_decreasing_array;
pub mod problem_0667_beautiful_arrangement_ii;
pub mod problem_0668_kth_smallest_number_in_multiplication_table;
pub mod problem_0669_trim_a_binary_search_tree;
pub mod problem_0670_maximum_swap;
pub mod problem_0671_second_minimum_node_in_a_binary_tree;
pub mod problem_0672_bulb_switcher_ii;
pub mod problem_0673_number_of_longest_increasing_subsequence;
pub mod problem_0674_longest_continuous_increasing_subsequence;
pub mod problem_0675_cut_off_trees_for_golf_event;
pub mod problem_0676_implement_magic_dictionary;
pub mod problem_0677_map_sum_pairs;
pub mod problem_0678_valid_parenthesis_string;
pub mod problem_0679_24_game;
pub mod problem_0680_valid_palindrome_ii;
pub mod problem_0682_baseball_game;
pub mod problem_0684_redundant_connection;
pub mod problem_0685_redundant_connection_ii;
pub mod problem_0686_repeated_string_match;
pub mod problem_0687_longest_univalue_path;
pub mod problem_0688_knight_probability_in_chessboard;
pub mod problem_0689_maximum_sum_of_3_non_overlapping_subarrays;
pub mod problem_0691_stickers_to_spell_word;
pub mod problem_0692_top_k_frequent_words;
pub mod problem_0693_binary_number_with_alternating_bits;
pub mod problem_0695_max_area_of_island;
pub mod problem_0696_count_binary_substrings;
pub mod problem_0697_degree_of_an_array;
pub mod problem_0698_partition_to_k_equal_sum_subsets;
pub mod problem_0699_falling_squares;
pub mod problem_0700_search_in_a_binary_search_tree;
pub mod problem_0701_insert_into_a_binary_search_tree;
pub mod problem_0703_kth_largest_element_in_a_stream;
pub mod problem_0704_binary_search;
pub mod problem_0705_design_hashset;
pub mod problem_0706_design_hashmap;
pub mod problem_0707_design_linked_list;
pub mod problem_0709_to_lower_case;
pub mod problem_0712_minimum_ascii_delete_sum_for_two_strings;
pub mod problem_0713_subarray_product_less_than_k;
pub mod problem_0714_best_time_to_buy_and_sell_stock_with_transaction_fee;
pub mod problem_0715_range_module;
pub mod problem_0717_1_bit_and_2_bit_characters;
pub mod problem_0718_maximum_length_of_repeated_subarray;
pub mod problem_0719_find_k_th_smallest_pair_distance;
pub mod problem_0720_longest_word_in_dictionary;
pub mod problem_0721_accounts_merge;
pub mod problem_0722_remove_comments;
pub mod problem_0724_find_pivot_index;
pub mod problem_0725_split_linked_list_in_parts;
pub mod problem_0726_number_of_atoms;
pub mod problem_0728_self_dividing_numbers;
pub mod problem_0729_my_calendar_i;
pub mod problem_0730_count_different_palindromic_subsequences;
pub mod problem_0731_my_calendar_ii;
pub mod problem_0732_my_calendar_iii;
pub mod problem_0733_flood_fill;
pub mod problem_0735_asteroid_collision;
pub mod problem_0736_parse_lisp_expression;
pub mod problem_0738_monotone_increasing_digits;
pub mod problem_0739_daily_temperatures;
pub mod problem_0740_delete_and_earn;
pub mod problem_0741_cherry_pickup;
pub mod problem_0743_network_delay_time;
pub mod problem_0744_find_smallest_letter_greater_than_target;
pub mod problem_0745_prefix_and_suffix_search;
pub mod problem_0746_min_cost_climbing_stairs;
pub mod problem_0747_largest_number_at_least_twice_of_others;
pub mod problem_0748_shortest_completing_word;
pub mod problem_0749_contain_virus;
pub mod problem_0752_open_the_lock;
pub mod problem_0753_cracking_the_safe;
pub mod problem_0754_reach_a_number;
pub mod problem_0756_pyramid_transition_matrix;
pub mod problem_0757_set_intersection_size_at_least_two;
pub mod problem_0761_special_binary_string;
pub mod problem_0762_prime_number_of_set_bits_in_binary_representation;
pub mod problem_0763_partition_labels;
pub mod problem_0764_largest_plus_sign;
pub mod problem_0765_couples_holding_hands;
pub mod problem_0766_toeplitz_matrix;
pub mod problem_0767_reorganize_string;
pub mod problem_0768_max_chunks_to_make_sorted_ii;
pub mod problem_0769_max_chunks_to_make_sorted;
pub mod problem_0770_basic_calculator_iv;
pub mod problem_0771_jewels_and_stones;
pub mod problem_0773_sliding_puzzle;
pub mod problem_0775_global_and_local_inversions;
pub mod problem_0777_swap_adjacent_in_lr_string;
pub mod problem_0778_swim_in_rising_water;
pub mod problem_0779_k_th_symbol_in_grammar;
pub mod problem_0780_reaching_points;
pub mod problem_0781_rabbits_in_forest;
pub mod problem_0782_transform_to_chessboard;
pub mod problem_0783_minimum_distance_between_bst_nodes;
pub mod problem_0784_letter_case_permutation;
pub mod problem_0785_is_graph_bipartite;
pub mod problem_0786_k_th_smallest_prime_fraction;
pub mod problem_0787_cheapest_flights_within_k_stops;
pub mod problem_0788_rotated_digits;
pub mod problem_0789_escape_the_ghosts;
pub mod problem_0790_domino_and_tromino_tiling;
pub mod problem_0791_custom_sort_string;
pub mod problem_0792_number_of_matching_subsequences;
pub mod problem_0793_preimage_size_of_factorial_zeroes_function;
pub mod problem_0794_valid_tic_tac_toe_state;
pub mod problem_0795_number_of_subarrays_with_bounded_maximum;
pub mod problem_0796_rotate_string;
pub mod problem_0797_all_paths_from_source_to_target;
pub mod problem_0798_smallest_rotation_with_highest_score;
pub mod problem_0799_champagne_tower;
pub mod problem_0801_minimum_swaps_to_make_sequences_increasing;
pub mod problem_0802_find_eventual_safe_states;
pub mod problem_0803_bricks_falling_when_hit;
pub mod problem_0804_unique_morse_code_words;
pub mod problem_0805_split_array_with_same_average;
pub mod problem_0806_number_of_lines_to_write_string;
pub mod problem_0807_max_increase_to_keep_city_skyline;
pub mod problem_0808_soup_servings;
pub mod problem_0809_expressive_words;
pub mod problem_0810_chalkboard_xor_game;
pub mod problem_0811_subdomain_visit_count;
pub mod problem_0812_largest_triangle_area;
pub mod problem_0813_largest_sum_of_averages;
pub mod problem_0814_binary_tree_pruning;
pub mod problem_0815_bus_routes;
pub mod problem_0816_ambiguous_coordinates;
pub mod problem_0817_linked_list_components;
pub mod problem_0818_race_car;
pub mod problem_0819_most_common_word;
pub mod problem_0820_short_encoding_of_words;
pub mod problem_0821_shortest_distance_to_a_character;
pub mod problem_0822_card_flipping_game;
pub mod problem_0823_binary_trees_with_factors;
pub mod problem_0824_goat_latin;
pub mod problem_0825_friends_of_appropriate_ages;
pub mod problem_0826_most_profit_assigning_work;
pub mod problem_0827_making_a_large_island;
pub mod problem_0828_count_unique_characters_of_all_substrings_of_a_given_string;
pub mod problem_0829_consecutive_numbers_sum;
pub mod problem_0830_positions_of_large_groups;
pub mod problem_0831_masking_personal_information;
pub mod problem_0832_flipping_an_image;
pub mod problem_0833_find_and_replace_in_string;
pub mod problem_0834_sum_of_distances_in_tree;
pub mod problem_0835_image_overlap;
pub mod problem_0836_rectangle_overlap;
pub mod problem_0837_new_21_game;
pub mod problem_0838_push_dominoes;
pub mod problem_0839_similar_string_groups;
pub mod problem_0840_magic_squares_in_grid;
pub mod problem_0841_keys_and_rooms;
pub mod problem_0842_split_array_into_fibonacci_sequence;
pub mod problem_0843_guess_the_word;
pub mod problem_0844_backspace_string_compare;
pub mod problem_0845_longest_mountain_in_array;
pub mod problem_0846_hand_of_straights;
pub mod problem_0847_shortest_path_visiting_all_nodes;
pub mod problem_0848_shifting_letters;
pub mod problem_0849_maximize_distance_to_closest_person;
pub mod problem_0850_rectangle_area_ii;
pub mod problem_0851_loud_and_rich;
pub mod problem_0852_peak_index_in_a_mountain_array;
pub mod problem_0853_car_fleet;
pub mod problem_0854_k_similar_strings;
pub mod problem_0855_exam_room;
pub mod problem_0856_score_of_parentheses;
pub mod problem_0857_minimum_cost_to_hire_k_workers;
pub mod problem_0858_mirror_reflection;
pub mod problem_0859_buddy_strings;
pub mod problem_0860_lemonade_change;
pub mod problem_0861_score_after_flipping_matrix;
pub mod problem_0862_shortest_subarray_with_sum_at_least_k;
pub mod problem_0863_all_nodes_distance_k_in_binary_tree;
pub mod problem_0863_projection_area_of_3d_shapes;
pub mod problem_0864_shortest_path_to_get_all_keys;
pub mod problem_0865_smallest_subtree_with_all_the_deepest_nodes;
pub mod problem_0866_prime_palindrome;
pub mod problem_0867_transpose_matrix;
pub mod problem_0868_binary_gap;
pub mod problem_0869_reordered_power_of_2;
pub mod problem_0870_advantage_shuffle;
pub mod problem_0871_minimum_number_of_refueling_stops;
pub mod problem_0872_leaf_similar_trees;
pub mod problem_0873_length_of_longest_fibonacci_subsequence;
pub mod problem_0874_walking_robot_simulation;
pub mod problem_0875_koko_eating_bananas;
pub mod problem_0876_middle_of_the_linked_list;
pub mod problem_0877_stone_game;
pub mod problem_0878_nth_magical_number;
pub mod problem_0879_profitable_schemes;
pub mod problem_0880_decoded_string_at_index;
pub mod problem_0881_boats_to_save_people;
pub mod problem_0882_reachable_nodes_in_subdivided_graph;
pub mod problem_0883_projection_area_of_3d_shapes;
pub mod problem_0884_uncommon_words_from_two_sentences;
pub mod problem_0885_spiral_matrix_iii;
pub mod problem_0886_possible_bipartition;
pub mod problem_0887_super_egg_drop;
pub mod problem_0888_fair_candy_swap;
pub mod problem_0889_construct_binary_tree_from_preorder_and_postorder_traversal;
pub mod problem_0890_find_and_replace_pattern;
pub mod problem_0891_sum_of_subsequence_widths;
pub mod problem_0892_surface_area_of_3d_shapes;
pub mod problem_0893_groups_of_special_equivalent_strings;
pub mod problem_0894_all_possible_full_binary_trees;
pub mod problem_0895_maximum_frequency_stack;
pub mod problem_0896_monotonic_array;
pub mod problem_0897_increasing_order_search_tree;
pub mod problem_0898_bitwise_ors_of_subarrays;
pub mod problem_0899_orderly_queue;
pub mod problem_0900_rle_iterator;
pub mod problem_0901_online_stock_span;
pub mod problem_0902_numbers_at_most_n_given_digit_set;
pub mod problem_0903_valid_permutations_for_di_sequence;
pub mod problem_0904_fruit_into_baskets;
pub mod problem_0905_sort_array_by_parity;
pub mod problem_0906_super_palindromes;
pub mod problem_0907_sum_of_subarray_minimums;
pub mod problem_0908_smallest_range_i;
pub mod problem_0909_snakes_and_ladders;
pub mod problem_0910_smallest_range_ii;
pub mod problem_0911_online_election;
pub mod problem_0912_sort_an_array;
pub mod problem_0913_cat_and_mouse;
pub mod problem_0914_x_of_a_kind_in_a_deck_of_cards;
pub mod problem_0915_partition_array_into_disjoint_intervals;
pub mod problem_0916_word_subsets;
pub mod problem_0917_reverse_only_letters;
pub mod problem_0918_maximum_sum_circular_subarray;
pub mod problem_0919_complete_binary_tree_inserter;
pub mod problem_0920_number_of_music_playlists;
pub mod problem_0921_minimum_add_to_make_parentheses_valid;
pub mod problem_0922_sort_array_by_parity_ii;
pub mod problem_0923_3sum_with_multiplicity;
pub mod problem_0924_minimize_malware_spread;
pub mod problem_0925_long_pressed_name;
pub mod problem_0926_flip_string_to_monotone_increasing;
pub mod problem_0927_three_equal_parts;
pub mod problem_0928_minimize_malware_spread_ii;
pub mod problem_0929_unique_email_addresses;
pub mod problem_0930_binary_subarrays_with_sum;
pub mod problem_0931_minimum_falling_path_sum;
pub mod problem_0932_beautiful_array;
pub mod problem_0933_number_of_recent_calls;
pub mod problem_0934_shortest_bridge;
pub mod problem_0935_knight_dialer;
pub mod problem_0936_stamping_the_sequence;
pub mod problem_0937_reorder_data_in_log_files;
pub mod problem_0938_range_sum_of_bst;
pub mod problem_0939_minimum_area_rectangle;
pub mod problem_0940_distinct_subsequences_ii;
pub mod problem_0941_valid_mountain_array;
pub mod problem_0942_di_string_match;
pub mod problem_0943_find_the_shortest_superstring;
pub mod problem_0944_delete_columns_to_make_sorted;
pub mod problem_0945_minimum_increment_to_make_array_unique;
pub mod problem_0946_validate_stack_sequences;
pub mod problem_0947_most_stones_removed_with_same_row_or_column;
pub mod problem_0948_bag_of_tokens;
pub mod problem_0949_largest_time_for_given_digits;
pub mod problem_0950_reveal_cards_in_increasing_order;
pub mod problem_0951_flip_equivalent_binary_trees;
pub mod problem_0952_largest_component_size_by_common_factor;
pub mod problem_0953_verifying_an_alien_dictionary;
pub mod problem_0954_array_of_doubled_pairs;
pub mod problem_0955_delete_columns_to_make_sorted_ii;
pub mod problem_0956_tallest_billboard;
pub mod problem_0957_prison_cells_after_n_days;
pub mod problem_0958_check_completeness_of_a_binary_tree;
pub mod problem_0959_regions_cut_by_slashes;
pub mod problem_0960_delete_columns_to_make_sorted_iii;
pub mod problem_0961_n_repeated_element_in_size_2n_array;
pub mod problem_0962_maximum_width_ramp;
pub mod problem_0963_minimum_area_rectangle_ii;
pub mod problem_0964_least_operators_to_express_number;
pub mod problem_0965_univalued_binary_tree;
pub mod problem_0966_vowel_spellchecker;
pub mod problem_0967_numbers_with_same_consecutive_differences;
pub mod problem_0968_binary_tree_cameras;
pub mod problem_0969_pancake_sorting;
pub mod problem_0970_powerful_integers;
pub mod problem_0971_flip_binary_tree_to_match_preorder_traversal;
pub mod problem_0972_equal_rational_numbers;
pub mod problem_0973_k_closest_points_to_origin;
pub mod problem_0974_subarray_sums_divisible_by_k;
pub mod problem_0975_odd_even_jump;
pub mod problem_0976_largest_perimeter_triangle;
pub mod problem_0977_squares_of_a_sorted_array;
pub mod problem_0978_longest_turbulent_subarray;
pub mod problem_0979_distribute_coins_in_binary_tree;
pub mod problem_0980_unique_paths_iii;
pub mod problem_0981_time_based_key_value_store;
pub mod problem_0982_triples_with_bitwise_and_equal_to_zero;
pub mod problem_0983_minimum_cost_for_tickets;
pub mod problem_0984_string_without_aaa_or_bbb;
pub mod problem_0985_sum_of_even_numbers_after_queries;
pub mod problem_0986_interval_list_intersections;
pub mod problem_0987_vertical_order_traversal_of_a_binary_tree;
pub mod problem_0988_smallest_string_starting_from_leaf;
pub mod problem_0989_add_to_array_form_of_integer;
pub mod problem_0990_satisfiability_of_equality_equations;
pub mod problem_0991_broken_calculator;
pub mod problem_0992_subarrays_with_k_different_integers;
pub mod problem_0993_cousins_in_binary_tree;
pub mod problem_0994_rotting_oranges;
pub mod problem_0995_minimum_number_of_k_consecutive_bit_flips;
pub mod problem_0996_number_of_squareful_arrays;
pub mod problem_0997_find_the_town_judge;
pub mod problem_0998_maximum_binary_tree_ii;
pub mod problem_0999_available_captures_for_rook;
pub mod problem_1000_minimum_cost_to_merge_stones;
pub mod problem_1001_grid_illumination;
pub mod problem_1002_find_common_characters;
pub mod problem_1003_check_if_word_is_valid_after_substitutions;
pub mod problem_1004_max_consecutive_ones_iii;
pub mod problem_1005_maximize_sum_of_array_after_k_negations;
pub mod problem_1006_clumsy_factorial;
pub mod problem_1007_minimum_domino_rotations_for_equal_row;
pub mod problem_1008_construct_binary_search_tree_from_preorder_traversal;
pub mod problem_1009_complement_of_base_10_integer;
pub mod problem_1010_pairs_of_songs_with_total_durations_divisible_by_60;
pub mod problem_1011_capacity_to_ship_packages_within_d_days;
pub mod problem_1012_numbers_with_repeated_digits;
pub mod problem_1013_partition_array_into_three_parts_with_equal_sum;
pub mod problem_1014_best_sightseeing_pair;
pub mod problem_1015_smallest_integer_divisible_by_k;
pub mod problem_1016_binary_string_with_substrings_representing_1_to_n;
pub mod problem_1017_convert_to_base_2;
pub mod problem_1018_binary_prefix_divisible_by_5;
pub mod problem_1019_next_greater_node_in_linked_list;
pub mod problem_1020_number_of_enclaves;
pub mod problem_1021_remove_outermost_parentheses;
pub mod problem_1022_sum_of_root_to_leaf_binary_numbers;
pub mod problem_1023_camelcase_matching;
pub mod problem_1024_video_stitching;
pub mod problem_1025_divisor_game;
pub mod problem_1026_maximum_difference_between_node_and_ancestor;
pub mod problem_1027_longest_arithmetic_subsequence;
pub mod problem_1028_recover_a_tree_from_preorder_traversal;
pub mod problem_1029_two_city_scheduling;
pub mod problem_1030_matrix_cells_in_distance_order;
pub mod problem_1031_maximum_sum_of_two_non_overlapping_subarrays;
pub mod problem_1032_stream_of_characters;
pub mod problem_1033_moving_stones_until_consecutive;
pub mod problem_1034_coloring_a_border;
pub mod problem_1035_uncrossed_lines;
pub mod problem_1036_escape_a_large_maze;
pub mod problem_1037_valid_boomerang;
pub mod problem_1038_binary_search_tree_to_greater_sum_tree;
pub mod problem_1039_minimum_score_triangulation_of_polygon;
pub mod problem_1040_moving_stones_until_consecutive_ii;
pub mod problem_1041_robot_bounded_in_circle;
pub mod problem_1042_flower_planting_with_no_adjacent;
pub mod problem_1043_partition_array_for_maximum_sum;
pub mod problem_1044_longest_duplicate_substring;
pub mod problem_1046_last_stone_weight;
pub mod problem_1047_remove_all_adjacent_duplicates_in_string;
pub mod problem_1048_longest_string_chain;
pub mod problem_1049_last_stone_weight_ii;
pub mod problem_1051_height_checker;
pub mod problem_1052_grumpy_bookstore_owner;
pub mod problem_1053_previous_permutation_with_one_swap;
pub mod problem_1054_distant_barcodes;
pub mod problem_1071_greatest_common_divisor_of_strings;
pub mod problem_1072_flip_columns_for_maximum_number_of_equal_rows;
pub mod problem_1073_adding_two_negabinary_numbers;
pub mod problem_1074_number_of_submatrices_that_sum_to_target;
pub mod problem_1078_occurrences_after_bigram;
pub mod problem_1079_letter_tile_possibilities;
pub mod problem_1080_insufficient_nodes_in_root_to_leaf_paths;
pub mod problem_1081_smallest_subsequence_of_distinct_characters;
pub mod problem_1089_duplicate_zeros;
pub mod problem_1090_largest_values_from_labels;
pub mod problem_1091_shortest_path_in_binary_matrix;
pub mod problem_1092_shortest_common_supersequence;
pub mod problem_1093_statistics_from_a_large_sample;
pub mod problem_1094_car_pooling;
pub mod problem_1095_find_in_mountain_array;
pub mod problem_1096_brace_expansion_ii;
pub mod problem_1103_distribute_candies_to_people;
pub mod problem_1104_path_in_zigzag_labelled_binary_tree;
pub mod problem_1105_filling_bookcase_shelves;
pub mod problem_1106_parsing_a_boolean_expression;
pub mod problem_1108_defanging_an_ip_address;
pub mod problem_1109_corporate_flight_bookings;
pub mod problem_1110_delete_nodes_and_return_forest;
pub mod problem_1111_maximum_nesting_depth_of_two_valid_parentheses_strings;
pub mod problem_1122_relative_sort_array;
pub mod problem_1123_lowest_common_ancestor_of_deepest_leaves;
pub mod problem_1124_longest_well_performing_interval;
pub mod problem_1125_smallest_sufficient_team;
pub mod problem_1128_number_of_equivalent_domino_pairs;
pub mod problem_1129_shortest_path_with_alternating_colors;
pub mod problem_1130_minimum_cost_tree_from_leaf_values;
pub mod problem_1131_maximum_of_absolute_value_expression;
pub mod problem_1137_n_th_tribonacci_number;
pub mod problem_1138_alphabet_board_path;
pub mod problem_1139_largest_1_bordered_square;
pub mod problem_1140_stone_game_ii;
pub mod problem_1143_longest_common_subsequence;
pub mod problem_1144_decrease_elements_to_make_array_zigzag;
pub mod problem_1145_binary_tree_coloring_game;
pub mod problem_1146_snapshot_array;
pub mod problem_1147_longest_chunked_palindrome_decomposition;
pub mod problem_1154_day_of_the_year;
pub mod problem_1155_number_of_dice_rolls_with_target_sum;
pub mod problem_1156_swap_for_longest_repeated_character_substring;
pub mod problem_1157_online_majority_element_in_subarray;
pub mod problem_1160_find_words_that_can_be_formed_by_characters;
pub mod problem_1161_maximum_level_sum_of_a_binary_tree;
pub mod problem_1162_as_far_from_land_as_possible;
pub mod problem_1163_last_substring_in_lexicographical_order;
pub mod problem_1169_invalid_transactions;
pub mod problem_1170_compare_strings_by_frequency_of_the_smallest_character;
pub mod problem_1171_remove_zero_sum_consecutive_nodes_from_linked_list;
pub mod problem_1172_dinner_plate_stacks;
pub mod problem_1175_prime_arrangements;
pub mod problem_1177_can_make_palindrome_from_substring;
pub mod problem_1178_number_of_valid_words_for_each_puzzle;
pub mod problem_1184_distance_between_bus_stops;
pub mod problem_1185_day_of_the_week;
pub mod problem_1186_maximum_subarray_sum_with_one_deletion;
pub mod problem_1187_make_array_strictly_increasing;
pub mod problem_1189_maximum_number_of_balloons;
pub mod problem_1190_reverse_substrings_between_each_pair_of_parentheses;
pub mod problem_1191_k_concatenation_maximum_sum;
pub mod problem_1192_critical_connections_in_a_network;
pub mod problem_1200_minimum_absolute_difference;
pub mod problem_1201_ugly_number_iii;
pub mod problem_1202_smallest_string_with_swaps;
pub mod problem_1203_sort_items_by_groups_respecting_dependencies;
pub mod problem_1206_design_skiplist;
pub mod problem_1207_unique_number_of_occurrences;
pub mod problem_1208_get_equal_substrings_within_budget;
pub mod problem_1209_remove_all_adjacent_duplicates_in_string_ii;
pub mod problem_1210_minimum_moves_to_reach_target_with_rotations;
pub mod problem_1217_minimum_cost_to_move_chips_to_the_same_position;
pub mod problem_1218_longest_arithmetic_subsequence_of_given_difference;
pub mod problem_1219_path_with_maximum_gold;
pub mod problem_1220_count_vowels_permutation;
pub mod problem_1221_split_a_string_in_balanced_strings;
pub mod problem_1222_queens_that_can_attack_the_king;
pub mod problem_1223_dice_roll_simulation;
pub mod problem_1224_maximum_equal_frequency;
pub mod problem_1227_airplane_seat_assignment_probability;
pub mod problem_1232_check_if_it_is_a_straight_line;
pub mod problem_1233_remove_sub_folders_from_the_filesystem;
pub mod problem_1234_replace_the_substring_for_balanced_string;
pub mod problem_1235_maximum_profit_in_job_scheduling;
pub mod problem_1237_find_positive_integer_solution_for_a_given_equation;
pub mod problem_1238_circular_permutation_in_binary_representation;
pub mod problem_1239_maximum_length_of_a_concatenated_string_with_unique_characters;
pub mod problem_1240_tiling_a_rectangle_with_the_fewest_squares;
pub mod problem_1247_minimum_swaps_to_make_strings_equal;
pub mod problem_1248_count_number_of_nice_subarrays;
pub mod problem_1249_minimum_remove_to_make_valid_parentheses;
pub mod problem_1250_check_if_it_is_a_good_array;
pub mod problem_1252_cells_with_odd_values_in_a_matrix;
pub mod problem_1253_reconstruct_a_2_row_binary_matrix;
pub mod problem_1254_number_of_closed_islands;
pub mod problem_1255_maximum_score_words_formed_by_letters;
pub mod problem_1260_shift_2d_grid;
pub mod problem_1261_find_elements_in_a_contaminated_binary_tree;
pub mod problem_1262_greatest_sum_divisible_by_three;
pub mod problem_1263_minimum_moves_to_move_a_box_to_their_target_location;
pub mod problem_1266_minimum_time_visiting_all_points;
pub mod problem_1267_count_servers_that_communicate;
pub mod problem_1268_search_suggestions_system;
pub mod problem_1269_number_of_ways_to_stay_in_the_same_place_after_some_steps;
pub mod problem_1275_find_winner_on_a_tic_tac_toe_game;
pub mod problem_1276_number_of_burgers_with_no_waste_of_ingredients;
pub mod problem_1277_count_square_submatrices_with_all_ones;
pub mod problem_1278_palindrome_partitioning_iii;
pub mod problem_1281_subtract_the_product_and_sum_of_digits_of_an_integer;
pub mod problem_1282_group_the_people_given_the_group_size_they_belong_to;
pub mod problem_1283_find_the_smallest_divisor_given_a_threshold;
pub mod problem_1284_minimum_number_of_flips_to_convert_binary_matrix_to_zero_matrix;
pub mod problem_1286_iterator_for_combination;
pub mod problem_1287_element_appearing_more_than_25_in_sorted_array;
pub mod problem_1288_remove_covered_intervals;
pub mod problem_1289_minimum_falling_path_sum_ii;
pub mod problem_1290_convert_binary_number_in_a_linked_list_to_integer;
pub mod problem_1291_sequential_digits;
pub mod problem_1292_maximum_side_length_of_a_square_with_sum_less_than_or_equal_to_threshold;
pub mod problem_1293_shortest_path_in_a_grid_with_obstacles_elimination;
pub mod problem_1295_find_numbers_with_even_number_of_digits;
pub mod problem_1296_divide_array_in_sets_of_k_consecutive_numbers;
pub mod problem_1297_maximum_number_of_occurrences_of_a_substring;
pub mod problem_1298_maximum_candies_you_can_get_from_boxes;
pub mod problem_1299_replace_elements_with_greatest_element_on_right_side;
pub mod problem_1300_sum_of_mutated_array_closest_to_target;
pub mod problem_1301_number_of_paths_with_max_score;
pub mod problem_1302_deepest_leaves_sum;
pub mod problem_1304_find_n_unique_integers_sum_up_to_zero;
pub mod problem_1305_all_elements_in_two_binary_search_trees;
pub mod problem_1306_jump_game_iii;
pub mod problem_1307_verbal_arithmetic_puzzle;
pub mod problem_1309_decrypt_string_from_alphabet_to_integer_mapping;
pub mod problem_1310_xor_queries_of_a_subarray;
pub mod problem_1311_get_watched_videos_by_your_friends;
pub mod problem_1312_minimum_insertion_steps_to_make_a_string_palindrome;
pub mod problem_1313_decompress_run_length_encoded_list;
pub mod problem_1314_matrix_block_sum;
pub mod problem_1315_sum_of_nodes_with_even_valued_grandparent;
pub mod problem_1316_distinct_echo_substrings;
pub mod problem_1317_convert_integer_to_the_sum_of_two_no_zero_integers;
pub mod problem_1318_minimum_flips_to_make_a_or_b_equal_to_c;
pub mod problem_1319_number_of_operations_to_make_network_connected;
pub mod problem_1320_minimum_distance_to_type_a_word_using_two_fingers;
pub mod problem_1323_maximum_69_number;
pub mod problem_1324_print_words_vertically;
pub mod problem_1325_delete_leaves_with_a_given_value;
pub mod problem_1326_minimum_number_of_taps_to_open_to_water_a_garden;
pub mod problem_1328_break_a_palindrome;
pub mod problem_1329_sort_the_matrix_diagonally;
pub mod problem_1330_reverse_subarray_to_maximize_array_value;
pub mod problem_1331_rank_transform_of_an_array;
pub mod problem_1332_remove_palindromic_subsequences;
pub mod problem_1333_filter_restaurants_by_vegan_friendly_price_and_distance;
pub mod problem_1334_find_the_city_with_the_smallest_number_of_neighbors_at_a_threshold_distance;
pub mod problem_1335_minimum_difficulty_of_a_job_schedule;
pub mod problem_1337_the_k_weakest_rows_in_a_matrix;
pub mod problem_1338_reduce_array_size_to_the_half;
pub mod problem_1339_maximum_product_of_splitted_binary_tree;
pub mod problem_1340_jump_game_v;
pub mod problem_1342_number_of_steps_to_reduce_a_number_to_zero;
pub mod problem_1343_number_of_sub_arrays_of_size_k_and_average_greater_than_or_equal_to_threshold;
pub mod problem_1344_angle_between_hands_of_a_clock;
pub mod problem_1345_jump_game_iv;
pub mod problem_1346_check_if_n_and_its_double_exist;
pub mod problem_1347_minimum_number_of_steps_to_make_two_strings_anagram;
pub mod problem_1348_tweet_counts_per_frequency;
pub mod problem_1349_maximum_students_taking_exam;
pub mod problem_1351_count_negative_numbers_in_a_sorted_matrix;
pub mod problem_1352_product_of_the_last_k_numbers;
pub mod problem_1353_maximum_number_of_events_that_can_be_attended;
pub mod problem_1354_construct_target_array_with_multiple_sums;
pub mod problem_1356_sort_integers_by_the_number_of_1_bits;
pub mod problem_1357_apply_discount_every_n_orders;
pub mod problem_1358_number_of_substrings_containing_all_three_characters;
pub mod problem_1359_count_all_valid_pickup_and_delivery_options;
pub mod problem_1360_number_of_days_between_two_dates;
pub mod problem_1361_validate_binary_tree_nodes;
pub mod problem_1362_closest_divisors;
pub mod problem_1363_largest_multiple_of_three;
pub mod problem_1365_how_many_numbers_are_smaller_than_the_current_number;
pub mod problem_1366_rank_teams_by_votes;
pub mod problem_1367_linked_list_in_binary_tree;
pub mod problem_1368_minimum_cost_to_make_at_least_one_valid_path_in_a_grid;
pub mod problem_1370_increasing_decreasing_string;
pub mod problem_1371_find_the_longest_substring_containing_vowels_in_even_counts;
pub mod problem_1372_longest_zigzag_path_in_a_binary_tree;
pub mod problem_1373_maximum_sum_bst_in_binary_tree;
pub mod problem_1374_generate_a_string_with_characters_that_have_odd_counts;
pub mod problem_1375_number_of_times_binary_string_is_prefix_aligned;
pub mod problem_1376_time_needed_to_inform_all_employees;
pub mod problem_1377_frog_position_after_t_seconds;
pub mod problem_1380_lucky_numbers_in_a_matrix;
pub mod problem_1381_design_a_stack_with_increment_operation;
pub mod problem_1382_balance_a_binary_search_tree;
pub mod problem_1383_maximum_performance_of_a_team;
pub mod problem_1385_find_the_distance_value_between_two_arrays;
pub mod problem_1386_cinema_seat_allocation;
pub mod problem_1387_sort_integers_by_the_power_value;
pub mod problem_1388_pizza_with_3n_slices;
pub mod problem_1389_create_target_array_in_the_given_order;
pub mod problem_1390_four_divisors;
pub mod problem_1391_check_if_there_is_a_valid_path_in_a_grid;
pub mod problem_1392_longest_happy_prefix;
pub mod problem_1394_find_lucky_integer_in_an_array;
pub mod problem_1395_count_number_of_teams;
pub mod problem_1396_design_underground_system;
pub mod problem_1399_count_largest_group;
pub mod problem_1400_construct_k_palindrome_strings;
pub mod problem_1401_circle_and_rectangle_overlapping;
pub mod problem_1402_reducing_dishes;
pub mod problem_1403_minimum_subsequence_in_non_increasing_order;
pub mod problem_1404_number_of_steps_to_reduce_a_number_in_binary_representation_to_one;
pub mod problem_1405_longest_happy_string;
pub mod problem_1406_stone_game_iii;
pub mod problem_1408_string_matching_in_an_array;
pub mod problem_1410_html_entity_parser;
pub mod problem_1411_number_of_ways_to_paint_n_3_grid;
pub mod problem_1413_minimum_value_to_get_positive_step_by_step_sum;
pub mod problem_1414_find_the_minimum_number_of_fibonacci_numbers_whose_sum_is_k;
pub mod problem_1415_the_k_th_lexicographical_string_of_all_happy_strings_of_length_n;
pub mod problem_1417_reformat_the_string;
pub mod problem_1418_display_table_of_food_orders_in_a_restaurant;
pub mod problem_1419_minimum_number_of_frogs_croaking;
pub mod problem_1422_maximum_score_after_splitting_a_string;
pub mod problem_1423_maximum_points_you_can_obtain_from_cards;
pub mod problem_1433_check_if_a_string_can_break_another_string;
pub mod problem_1437_check_if_all_1s_are_at_least_length_k_places_away;
pub mod problem_1438_longest_continuous_subarray_with_absolute_diff_less_than_or_equal_to_limit;
pub mod problem_1441_build_an_array_with_stack_operations;
pub mod problem_1720_decode_xored_array;
pub mod problem_1736_latest_time_by_replacing_hidden_digits;
pub mod problem_1743_restore_the_array_from_adjacent_pairs;
pub mod problem_1752_check_if_array_is_sorted_and_rotated;
pub mod problem_1773_count_items_matching_a_rule;

#[cfg(test)]
mod test_utilities;
