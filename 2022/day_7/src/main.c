#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <ctype.h>

enum CHALLENGE_NUMBER {
    FIRST_CHALLENGE = 1,
    SECOND_CHALLENGE = 2,
};

typedef enum {
    UNKNOWN,
    LIST_FILES,
    CHANGE_DIRECTORY,
} command_t;

typedef struct command_line {
    command_t command;
    char arg[255];
} command_line_t;

typedef struct file {
    char name[255];
    int size;
} file_t;

typedef struct dir {
    char name[255];
    struct dir *parent;
    int subdirectories_size;
    struct dir **subdirectories;
    int files_size;
    file_t **files;
} dir_t;

typedef enum {
    LS_ERROR,
    LS_DIRECTORY,
    LS_FILE,
} ls_output_type_t;

typedef struct ls_output {
    ls_output_type_t type;
    char name[255];
    int size;
} ls_output_t;

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

void print_directory(dir_t *dir)
{
    fprintf(stdout, "[DEBUG] Directory \"%s\"\n", dir->name);
    fprintf(stdout, "[DEBUG] \tSubdirectories (%d):\n", dir->subdirectories_size);
    for (int i = 0; i < dir->subdirectories_size; i++) {
        fprintf(stdout, "[DEBUG] \t\t\"%s\"\n", dir->subdirectories[i]->name);
    }

    fprintf(stdout, "[DEBUG] \tFiles (%d):\n", dir->files_size);
    for (int i = 0; i < dir->files_size; i++) {
        fprintf(stdout, "[DEBUG] \t\t\"%s\": %d bytes\n", dir->files[i]->name, dir->files[i]->size);
    }

}

char *trim(char *str)
{
    char *end;
    // Trim leading space
    while(isspace((unsigned char)*str)) {
        str++;
    }

    if(*str == 0) {
        // All spaces?
        return str;
    }

    // Trim trailing space
    end = str + strlen(str) - 1;
    while(end > str && isspace((unsigned char)*end)) {
        end--;
    }

    // Write new null terminator character
    end[1] = '\0';
    return str;
}

command_line_t parse_command(char *buffer) {
    char err_buf[512] = { 0 };
    char tmp[255] = { 0 };
    strncpy(tmp, buffer, sizeof(tmp));

    command_line_t ret = { UNKNOWN, { 0 } };

    char *token = strtok(tmp, " ");
    int token_count = 0;
    while (token != NULL) {
        // printf("token: %s\n", token);
        if (token_count == 0) {
            if (strncmp(token, "$", strlen(token))) {
                snprintf(err_buf, sizeof(err_buf), "[ERROR] Not a command: %s", buffer);
                goto ERROR;
            }
        } else if (token_count == 1) {
            // Command type
            if (!strncmp(token, "cd", strlen(token))) {
                ret.command = CHANGE_DIRECTORY;
            } else if (!strncmp(token, "ls", strlen(token))) {
                ret.command = LIST_FILES;
            } else {
                snprintf(err_buf, sizeof(err_buf), "[ERROR] Unsupported command %s", buffer);
                goto ERROR;
            }
        } else if (token_count == 2) {
            if (ret.command == LIST_FILES) {
                snprintf(err_buf, sizeof(err_buf), "[ERROR] ls with arguments not supported: %s", buffer);
                goto ERROR;
            }
            // Command argument
            strncpy(ret.arg, token, strlen(token));
        } else {
            snprintf(err_buf, sizeof(err_buf), "[ERROR] Too many argument in command %s", buffer);
            goto ERROR;
        }
        // Next token
        token = strtok(NULL, " ");
        token_count++;
    }

    return ret;
ERROR:
    fprintf(stderr, "%s\n", err_buf);
    ret.command = UNKNOWN;
    memset(ret.arg, 0, sizeof(ret.arg));
    return ret;
}

dir_t *change_directory(dir_t *from, char *to)
{
    if (from == NULL) {
        fprintf(stderr, "[ERROR] Cannot \"cd\" from NULL\n");
        return NULL;
    }

    if (!strncmp(to, "..", strlen(to))) {
        // Go to parent directory
        if (from->parent == NULL) {
            fprintf(stderr, "[ERROR] \"%s\" has no parent directory\n", from->name);
            return NULL;
        }
        return from->parent;

    }

    // Find subdirectory
    for (int i = 0; i < from->subdirectories_size; i++) {
        if (!strncmp(from->subdirectories[i]->name, to, sizeof(from->subdirectories[i]->name))) {
            return from->subdirectories[i];
        }
    }

    fprintf(stderr, "[ERROR] \"%s\" is not a subdirectory of \"%s\"\n", to, from->name);
    return NULL;
}

ls_output_t parse_ls_output(char *buffer)
{
    char tmp[255] = { 0 };
    strncpy(tmp, buffer, sizeof(tmp));

    ls_output_t err = { LS_ERROR, { 0 }, 0 };
    ls_output_t ret = err;

    char *token = strtok(tmp, " ");
    int token_count = 1;
    while (token != NULL) {
        switch (token_count) {
            case 1:
                if (!strncmp(token, "dir", strlen(token))) {
                    ret.type = LS_DIRECTORY;
                } else {
                    char *end = NULL;
                    ret.size = strtol(token, &end, 10);
                    if (end == token || *end != '\0') {
                        fprintf(stderr, "[ERROR] %s is not a valid \"ls\" output\n", buffer);
                        return err;
                    }
                    ret.type = LS_FILE;
                }
                break;
            case 2:
                strncpy(ret.name, token, sizeof(ret.name));
                break;
            default:
                fprintf(stderr, "[ERROR] %s is not a valid \"ls\" output\n", buffer);
                return err;
        }
        token = strtok(NULL, " ");
        token_count++;
    }

    return ret;
}

void add_subdirectory(dir_t *dir, char *subdir_name)
{
    for (int i = 0; i < dir->subdirectories_size; i++) {
        if (!strncmp(dir->subdirectories[i]->name, subdir_name, sizeof(dir->subdirectories[i]->name))) {
            // Subdir already added
            return;
        }
    }

    dir_t *subdir = calloc(1, sizeof(*subdir));
    if (subdir == NULL) {
        fprintf(stderr, "[ERROR] malloc returned NULL\n");
        return;
    }

    dir_t **tmp = realloc(dir->subdirectories, (dir->subdirectories_size + 1) * sizeof(*dir->subdirectories));
    if (tmp == NULL) {
        fprintf(stderr, "[ERROR] realloc returned NULL\n");
        return;
    }
    dir->subdirectories = tmp;

    strncpy(subdir->name, subdir_name, sizeof(subdir->name));
    subdir->parent = dir;
    dir->subdirectories[dir->subdirectories_size] = subdir;
    dir->subdirectories_size++;
}

void add_file(dir_t *dir, char *file_name, int file_size)
{
    for (int i = 0; i < dir->files_size; i++) {
        if (!strncmp(dir->files[i]->name, file_name, sizeof(dir->files[i]->name))) {
            // File already added
            return;
        }
    }

    file_t *file = calloc(1, sizeof(*file));
    if (file == NULL) {
        fprintf(stderr, "[ERROR] malloc returned NULL\n");
        return;
    }

    file_t **tmp = realloc(dir->files, (dir->files_size + 1) * sizeof(*dir->files));
    if (tmp == NULL) {
        fprintf(stderr, "[ERROR] realloc returned NULL\n");
        return;
    }
    dir->files = tmp;

    strncpy(file->name, file_name, sizeof(file->name));
    file->size = file_size;
    dir->files[dir->files_size] = file;
    dir->files_size++;
}

void list_file(dir_t *dir, ls_output_t ls_result)
{
    if (dir == NULL) {
        fprintf(stderr, "[ERROR] Cannot call \"ls\" while in NULL\n");
        exit(EXIT_FAILURE);
    }

    switch (ls_result.type) {
        case LS_DIRECTORY:
            add_subdirectory(dir, ls_result.name);
            break;
        case LS_FILE:
            add_file(dir, ls_result.name, ls_result.size);
            break;
        default:
            fprintf(stderr, "[ERROR] ls outputed an error\n");
            exit(EXIT_FAILURE);
    }
}

void cleanup_filesystem(dir_t *dir)
{
    if (dir == NULL) {
        return;
    }

    for (int i = 0; i < dir->subdirectories_size; i++) {
        cleanup_filesystem(dir->subdirectories[i]);
    }

    for (int i = 0; i < dir->files_size; i++) {
        free(dir->files[i]);
    }
    if (dir->files != NULL) {
        free(dir->files);
    }

    if (dir->subdirectories != NULL) {
        free(dir->subdirectories);
    }
    free(dir);
}

dir_t *parse_input_file(char *filename)
{
    char err_buf[512] = { 0 };

    FILE *fp = fopen(filename, "r");
    if (fp == NULL) {
        fprintf(stderr, "[ERROR] Failed to open %s\n", filename);
        exit(EXIT_FAILURE);
    }

    char buffer[255] = { 0 };

    dir_t *root = calloc(1, sizeof(*root));
    if (root == NULL) {
        fprintf(stderr, "[ERROR] malloc returned NULL\n");
    }
    strncpy(root->name, "/", sizeof(root->name));

    dir_t *current_directory = root;

    command_line_t command;
    command.command = UNKNOWN;

    bool first_line = true;
    while (!feof(fp)) {
        // Debug
        // print_directory(current_directory);
        memset(buffer, 0, sizeof(buffer));
        // Read line
        fgets(buffer, sizeof(buffer), fp);
        trim(buffer);
        bool changed_command = false;

        // Skip first line (always "$ cd /")
        if (first_line) {
            first_line = false;
            continue;
        }

        if (strlen(buffer) >= 1 && !strncmp(buffer, "$", 1)) {
            command = parse_command(buffer);
            changed_command = true;
        }

        switch (command.command) {
            case CHANGE_DIRECTORY:
                current_directory = change_directory(current_directory, command.arg);
                if (current_directory == NULL) {
                    snprintf(err_buf, sizeof(err_buf), "[ERROR] Failed to change directory");
                    goto ERROR;
                }
                break;
            case LIST_FILES: ;
                if (changed_command) {
                    continue;
                }
                ls_output_t ls_result = parse_ls_output(buffer);
                list_file(current_directory, ls_result);
                break;
            default:
                snprintf(err_buf, sizeof(err_buf), "[ERROR] Got an output without a valid command: %s", buffer);
                goto ERROR;
        }
    }
    fclose(fp);
    return root;
ERROR:
    fclose(fp);
    fprintf(stderr, "%s\n", err_buf);
    return NULL;
}

int calculate_freeable_space(dir_t *dir, int threshold, int *deleted)
{
    int dir_size = 0;

    for (int i = 0; i < dir->subdirectories_size; i++) {
        dir_size += calculate_freeable_space(dir->subdirectories[i], threshold, deleted);
    }

    for (int i = 0; i < dir->files_size; i++) {
        dir_size += dir->files[i]->size;
    }

    if (dir_size <= threshold && deleted != NULL) {
        *deleted += dir_size;
    }
    return dir_size;
}

int get_smallest_dir_above_limit(dir_t *dir, int limit, int *best_to_delete)
{
    int dir_size = 0;

    for (int i = 0; i < dir->subdirectories_size; i++) {
        dir_size += get_smallest_dir_above_limit(dir->subdirectories[i], limit, best_to_delete);
    }

    for (int i = 0; i < dir->files_size; i++) {
        dir_size += dir->files[i]->size;
    }

    if  (best_to_delete != NULL) {
        if (dir_size < *best_to_delete && dir_size > limit) {
            *best_to_delete = dir_size;
        }
    }
    return dir_size;
}

void solve_challenge_1(char *filename)
{
    dir_t *root = parse_input_file(filename);
    if (root == NULL) {
        fprintf(stderr, "[ERROR] Failed to parse input\n");
        exit(EXIT_FAILURE);
    }

    int threshold = 100000;
    int ans = 0;
    calculate_freeable_space(root, threshold, &ans);

    fprintf(stdout, "Answer : %d\n", ans);
    cleanup_filesystem(root);
    return;
}

void solve_challenge_2(char *filename)
{
    dir_t *root = parse_input_file(filename);
    if (root == NULL) {
        fprintf(stderr, "[ERROR] Failed to parse input\n");
        exit(EXIT_FAILURE);
    }

    int filesystem_size = get_smallest_dir_above_limit(root, 0, NULL);
    int limit = filesystem_size - (70000000 - 30000000);
    int ans = 70000000;
    if (limit <= 0) {
        ans = 0;
    } else {
        get_smallest_dir_above_limit(root, limit, &ans);
    }

    fprintf(stdout, "Answer : %d\n", ans);
    cleanup_filesystem(root);
    return;
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