#![warn(
    explicit_outlives_requirements,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    noop_method_call,
    pointer_structural_match,
    semicolon_in_expressions_from_macros,
    // trivial_casts,
    trivial_numeric_casts,
    unaligned_references,
    unsafe_op_in_unsafe_fn,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    clippy::clone_on_ref_ptr,
    clippy::cognitive_complexity,
    clippy::debug_assert_with_mut_call,
    clippy::empty_line_after_outer_attr,
    clippy::fallible_impl_from,
    clippy::get_unwrap,
    clippy::imprecise_flops,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::multiple_inherent_impl,
    clippy::mutex_integer,
    clippy::needless_borrow,
    clippy::panic_in_result_fn,
    clippy::path_buf_push_overwrite,
    clippy::pedantic,
    clippy::rc_buffer,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::semicolon_if_nothing_returned,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::suboptimal_flops,
    clippy::todo,
    clippy::trivial_regex,
    clippy::unimplemented,
    clippy::unneeded_field_pattern,
    clippy::use_debug,
    clippy::use_self,
    clippy::useless_let_if_seq,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    clippy::wrong_pub_self_convention
)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::non_ascii_literal,
    clippy::unnested_or_patterns
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
pub mod problem_0028_implement_strstr;
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
pub mod problem_0518_coin_change_2;
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
pub mod problem_0561_array_partition_i;
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
pub mod problem_0686_repeated_string_match;
pub mod problem_0687_longest_univalue_path;
pub mod problem_0688_knight_probability_in_chessboard;
pub mod problem_0689_maximum_sum_of_3_non_overlapping_subarrays;
pub mod problem_0692_top_k_frequent_words;
pub mod problem_0693_binary_number_with_alternating_bits;
pub mod problem_0695_max_area_of_island;
pub mod problem_0704_binary_search;
pub mod problem_0709_to_lower_case;
pub mod problem_0721_accounts_merge;
pub mod problem_0733_flood_fill;
pub mod problem_0739_daily_temperatures;
pub mod problem_0763_partition_labels;
pub mod problem_0785_is_graph_bipartite;
pub mod problem_0797_all_paths_from_source_to_target;
pub mod problem_0832_flipping_an_image;
pub mod problem_0867_transpose_matrix;
pub mod problem_1143_longest_common_subsequence;
pub mod problem_1192_critical_connections_in_a_network;
pub mod problem_1342_number_of_steps_to_reduce_a_number_to_zero;
pub mod problem_1720_decode_xored_array;
pub mod problem_1736_latest_time_by_replacing_hidden_digits;
pub mod problem_1743_restore_the_array_from_adjacent_pairs;
pub mod problem_1752_check_if_array_is_sorted_and_rotated;
pub mod problem_1773_count_items_matching_a_rule;

#[cfg(test)]
mod test_utilities;
