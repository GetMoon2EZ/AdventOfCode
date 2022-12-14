#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <ctype.h>

#define BUFFER_SIZE     256

enum CHALLENGE_NUMBER {
    FIRST_CHALLENGE = 1,
    SECOND_CHALLENGE = 2,
};

typedef enum {
    INNER_LIST,
    INTEGER,
} list_order_t;

typedef struct list {
    /// @brief Outer list (or parent)
    struct list *outer;
    /// @brief Order if which to process elements
    list_order_t *order;
    /// @brief Number of inner lists (non recursive)
    uint32_t inner_count;
    /// @brief Inner lists (or children)
    struct list **inner;
    /// @brief Number of integer in list (non recursive)
    uint32_t ints_count;
    /// @brief Integers
    int32_t *ints;
} list_t;

void usage()
{
    fprintf(stdout, "Usage: advent_of_code <CHALLENGE_NUM> <FILENAME>\n\n");
    fprintf(stdout, "Arguments:\n");
    fprintf(stdout, "  <CHALLENGE_NUM>  Challenge to run (1 or 2)\n");
    fprintf(stdout, "  <FILENAME>       Input file\n\n");
    fprintf(stdout, "Options:\n");
    fprintf(stdout, "  -h, --help  Print help information\n");
    fprintf(stdout, "  <CHALLENGE_NUM>  Challenge to run (1 or 2)\n");
}

char *trim(char *str)
{
    char *end;
    // Trim leading space
    while(isspace(*str)) {
        str++;
    }

    if(*str == 0) {
        // All spaces?
        return str;
    }

    // Trim trailing space
    end = str + strlen(str) - 1;
    while(end > str && isspace(*end)) {
        end--;
    }

    // Write new null terminator character
    end[1] = '\0';
    return str;
}

void cleanup_list(list_t *list) {
    if (list == NULL) {
        return;
    }

    if (list->ints != NULL) {
        free(list->ints);
    }

    if (list->inner != NULL) {
        for (uint32_t i = 0; i < list->inner_count; i++) {
            cleanup_list(list->inner[i]);
        }
        free(list->inner);
    }

    if (list->order != NULL) {
        free(list->order);
    }
    free(list);
}

int push_order_to_list(list_t *list, list_order_t order)
{
    if (list == NULL) {
        fprintf(stderr, "[ERROR] Cannot push order to NULL\n");
        return EXIT_FAILURE;
    }

    // Last element in the order is the current inner list
    uint32_t order_count = list->inner_count + list->ints_count;
    list_order_t *tmp_order = realloc(list->order, (order_count + 1) * sizeof(*(list->order)));
    if (tmp_order == NULL) {
        fprintf(stderr, "[ERROR] realloc returned NULL\n");
        return EXIT_FAILURE;
    }
    list->order = tmp_order;
    list->order[order_count] = order;
    return EXIT_SUCCESS;
}

list_t* push_list_to_list(list_t *list)
{
    if (list == NULL) {
        fprintf(stderr, "[ERROR] Cannot push list to NULL\n");
        return NULL;
    }

    if (push_order_to_list(list, INNER_LIST) != EXIT_SUCCESS) {
        fprintf(stderr, "[ERROR] Failed to push order INNER_LIST\n");
        return NULL;
    }

    uint32_t index = list->inner_count;
    list_t **tmp_inner = realloc(list->inner, (index + 1) * sizeof(*(list->inner)));
    if (tmp_inner == NULL) {
        fprintf(stderr, "[ERROR] realloc returned NULL\n");
        return NULL;
    }
    list->inner = tmp_inner;

    list->inner[index] = calloc(1, sizeof(*(list->inner[index])));
    if (list->inner[index] == NULL) {
        fprintf(stderr, "[ERROR] malloc returned NULL\n");
        return NULL;
    }

    list->inner[index]->outer = list;
    return list->inner[list->inner_count++];
}

void push_int_to_list(list_t *list, uint32_t number)
{
    if (list == NULL) {
        fprintf(stderr, "[ERROR] Cannot push %d to NULL\n", number);
        exit(EXIT_FAILURE);
    }

    if (push_order_to_list(list, INTEGER) != EXIT_SUCCESS) {
        fprintf(stderr, "[ERROR] Failed to push order INTEGER\n");
        exit(EXIT_FAILURE);
    }

    int32_t *tmp = realloc(list->ints, (list->ints_count + 1) * sizeof(*(list->ints)));
    if (tmp == NULL) {
        fprintf(stderr, "[ERROR] realloc returned NULL\n");
        exit(EXIT_FAILURE);
    }
    list->ints = tmp;
    list->ints[list->ints_count++] = number;
}

list_t* parse_packet(const char *line) {
    char buffer[BUFFER_SIZE] = { 0 };
    uint32_t i_buf = 0;
    list_t *root = calloc(1, sizeof(list_t));
    list_t *current_list = root;

    for(uint32_t i = 0; i < strlen(line); i++) {
        char *end = NULL;
        int32_t number = 0;
        switch (line[i]) {
            case '[':
                current_list = push_list_to_list(current_list);
                if (current_list == NULL) {
                    fprintf(stderr, "[ERROR] Failed to add inner list !\n");
                    goto ERROR;
                }
                break;

            case ']':
                if (i_buf == 0) {
                    continue;
                }
                // Convert buffer to int
                number = (int32_t) strtol(buffer, &end, 10);
                if (end == buffer || *end != '\0') {
                    fprintf(stderr, "[ERROR] Failed to convert %s to uint32_t\n", buffer);
                    goto ERROR;
                }
                memset(buffer, 0, sizeof(buffer));
                i_buf = 0;
                // Push result to current list
                push_int_to_list(current_list, number);
                current_list = current_list->outer;
                break;

            case ',':
                if (i_buf == 0) {
                    continue;
                }
                number = (int32_t) strtol(buffer, &end, 10);
                if (end == buffer || *end != '\0') {
                    fprintf(stderr, "[ERROR] Failed to convert %s to uint32_t\n", buffer);
                    goto ERROR;
                }
                memset(buffer, 0, sizeof(buffer));
                i_buf = 0;
                push_int_to_list(current_list, number);
                break;

            default:
                buffer[i_buf++] = line[i];
        }
    }

    return root;

ERROR:
    cleanup_list(root);
    return NULL;
}

typedef struct param {
    // Index of the next list to process
    uint32_t i_list;
    // Index of the next int to process
    uint32_t i_int;
    // If true, treat current integer as a one size list
    bool as_list;
} param_t;

bool is_pair_in_order(list_t *left, list_t *right, param_t left_param, param_t right_param)
{
    // TODO pair comparison (recursive function)
    // uint32_t i_left = left_param.i_int + left_param.i_list;
    // uint32_t i_right = right_param.i_int + right_param.i_list;

    // // Left finished at the same time or earlier than right
    // if (i_left > left->inner_count + left->ints_count) {
    //     return true;
    // }

    // // Right finished earlier than left
    // if (i_right > right->inner_count + right->ints_count) {
    //     return false;
    // }

    // // If BOTH integers
    // if (left->order[i_left] == INTEGER && right->order[i_right] == INTEGER) {
    //     uint32_t l_int = left->ints[left_param.i_int];
    //     uint32_t r_int = right->ints[right_param.i_int];
    //     if (l_int > r_int) {
    //         return false;
    //     } else if (l_int > r_int) {
    //         return true;
    //     } else {
    //         left_param.i_int += 1;
    //         right_param.i_int += 1;
    //         return is_pair_in_order(left, right, left_param, right_param);
    //     }
    // }
    // if (left->order[i_left] == INNER_LIST && right->order[i_right] == INNER_LIST) {
    //     param_t default_param = { 0, 0, false };
    //     return is_pair_in_order(left->inner[left_param.i_list], right->inner[right_param.i_list], default_param, default_param);
    // }

    // One is a list, the other is an int
    return false;
}

void solve_challenge_1(const char *filename)
{
    FILE *fp = fopen(filename, "r");
    if (fp == NULL) {
        fprintf(stderr, "[ERROR] Unable to open file: %s\n", filename);
        exit(EXIT_FAILURE);
    }

    char buffer[BUFFER_SIZE] = { 0 };
    uint32_t nb_packets = 0;
    list_t **packets = NULL;
    // Parse input file
    while (!feof(fp)) {
        // Reset buffer
        memset(buffer, 0, sizeof(buffer));
        // Read line
        fgets(buffer, sizeof(buffer), fp);
        // Remove white spaces, return line etc...
        char *clean_buffer = trim(buffer);
        if (strlen(clean_buffer) == 0) {
            // Empty line between packet pairs
            continue;
        }

        list_t *packet = parse_packet((const char*) clean_buffer);
        if (packet == NULL) {
            fprintf(stderr, "[ERROR] Could not parse packet: %s\n", clean_buffer);
            goto ERROR;
        }

        list_t **tmp = realloc(packets, (nb_packets + 1) * sizeof(*packets));
        if (tmp == NULL) {
            free(packet);
            fprintf(stderr, "[ERROR] realloc returned NULL\n");
            goto ERROR;
        }
        packets = tmp;
        packets[nb_packets++] = packet;
    }

    if (nb_packets % 2 != 0) {
        fprintf(stderr, "[ERROR] Expected a pair number of packets, got %d\n", nb_packets);
        goto ERROR;
    }
    fclose(fp);

    uint32_t ans = 0;
    param_t default_param = { 0, 0, false };
    for (uint32_t i = 0; i < nb_packets; i+=2) {
        ans += is_pair_in_order(packets[i], packets[i + 1], default_param, default_param)? i : 0;
    }

    for (int i = 0; i < nb_packets; i++) {
        cleanup_list(packets[i]);
    }
    free(packets);

    fprintf(stdout, "Answer: %d\n", ans);
    return;

ERROR:
    fclose(fp);
    for (int i = 0; i < nb_packets; i++) {
        cleanup_list(packets[i]);
    }
    free(packets);
    exit(EXIT_FAILURE);
}

void solve_challenge_2(const char *filename)
{
    fprintf(stdout, "[INFO] Cannot solve challenge 2 on %s\n", filename);
    fprintf(stdout, "[INFO] Challenge 2 solution not implemented\n");
}

int main(int argc, char **argv)
{
    // Arg check
    if (
        argc == 2
        && (
            !strncmp(argv[1], "--help", strlen(argv[1]))
            || !strncmp(argv[1], "-h", strlen(argv[1]))
        )
    ) {
        usage();
        exit(EXIT_SUCCESS);
    }
    if (argc != 3) {
        fprintf(stderr, "[ERROR] Expected 2 arguments, got %d\n\n", argc - 1);
        usage();
        return EXIT_FAILURE;
    }

    char *tmp = NULL;
    int challenge_num = strtol(argv[1], &tmp, 10);
    if (tmp == argv[1] || *tmp != '\0') {
        fprintf(stderr, "[ERROR] Challenge number is not a number: %s\n\n", argv[1]);
        usage();
        exit(EXIT_FAILURE);
    }

    char filename[255] = { 0 };
    strncpy(filename, argv[2], sizeof(filename));

    switch (challenge_num) {
        case FIRST_CHALLENGE:
            solve_challenge_1(filename);
            break;
        case SECOND_CHALLENGE:
            solve_challenge_2(filename);
            break;
        default:
            fprintf(stderr, "[ERROR] Unexpected challenge number: %d\n\n", challenge_num);
            usage();
            return EXIT_FAILURE;
    }

    return EXIT_SUCCESS;
}