#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include <stdlib.h>
#include <time.h>
#include "cutil.h"

#define REG_RES(i, e) { if (strcmp(id, i) == 0) return e; }

struct game {
	int32_t id;
};

// int32_t* -> game*
HashTable* game_table;

int32_t new_game() {
	int32_t id = rand();
	struct game* g = malloc(sizeof *g);
	g->id = id;
	HashTableAdd(game_table, (byte*)&id, 4, g);
	return id;
}

void inspect(int32_t gameid) {
	struct game* g = HashTableSearch(game_table, (byte*)&gameid, 4);
	printf("Found game: 0x%X\n", g->id);
}

void* resource(char* id) {
	REG_RES("id", "test-lib")
	REG_RES("version", "0.1.0")
}

size_t id_hash(byte *key, size_t keylen) {
	int32_t k = *(int32_t*)(key);
	return (size_t)(k);
}

void init(void) {
	srand(time(NULL));
	game_table = HashTableAlloc(1024, true, id_hash);
}

void work(void) {
	int32_t id = rand();
	printf("gameid u64: 0x%X\n", id);
	printf("mxrand: 0x%X\n", RAND_MAX);
}

