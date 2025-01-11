#include <iostream>
#include <cstdint>
#include <vector>
#include <x86intrin.h>

constexpr uint64_t M[3][3] = {
	{ 0xaf888fd0c9634130, 0xe7c34220149b7728, 0xb00193b6071c8868 },
	{ 0x1917419c62e210ad, 0x553337b6adec81cf, 0x545796a28c730795 },
	{ 0x16463dbaced076c1, 0x46714f6cbc284527, 0x718462a50da6d92d },
};

static __m512i group_by_byte = _mm512_set_epi64(
	0x3f3b37332f2b2723, 0x1f1b17130f0b0703,
	0x3e3a36322e2a2622, 0x1e1a16120e0a0602,
	0x3d3935312d292521, 0x1d1915110d090501,
	0x3c3834302c282420, 0x1c1814100c080400);

static __m512i permute_solution = _mm512_permutexvar_epi8(group_by_byte, group_by_byte);

static __m512i matrix_a = _mm512_set_epi64(0, 0, M[2][2], M[2][2], M[1][1], M[1][1], M[0][0], M[0][0]);

static __m512i matrix_b = _mm512_set_epi64(0, 0, M[1][2], M[1][2], M[0][1], M[0][1], M[2][0], M[2][0]);
static __m512i permute_b = _mm512_set_epi64(7, 6, 1, 0, 5, 4, 3, 2);

static __m512i matrix_c = _mm512_set_epi64(0, 0, M[0][2], M[0][2], M[2][1], M[2][1], M[1][0], M[1][0]);
static __m512i permute_c = _mm512_set_epi64(7, 6, 3, 2, 1, 0, 5, 4);

__m512i solve(const uint32_t *A) {
	auto v = _mm512_loadu_epi32(A);

	v = _mm512_permutexvar_epi8(group_by_byte, v);

	auto a = _mm512_gf2p8affine_epi64_epi8(v, matrix_a, 0);
	auto b = _mm512_gf2p8affine_epi64_epi8(v, matrix_b, 0);
	auto c = _mm512_gf2p8affine_epi64_epi8(v, matrix_c, 0);

	b = _mm512_permutexvar_epi64(permute_b, b);
	c = _mm512_permutexvar_epi64(permute_c, c);

	return _mm512_permutexvar_epi8(permute_solution, a ^ b ^ c);
}

uint64_t horizontal_sum(__m512i v) {
	auto lo = _mm512_unpacklo_epi32(v, _mm512_setzero_si512());
	auto hi = _mm512_unpackhi_epi32(v, _mm512_setzero_si512());
	return _mm512_reduce_add_epi64(lo + hi);
}

int main() {
	std::vector<uint32_t> input;

	uint32_t n;
	while (std::cin >> n) {
		input.push_back(n);
	}

	auto count = input.size();
	input.resize(count + 16);

	__m512i v = _mm512_setzero_si512();
	for (int idx = 0; idx < count; idx += 16) {
		v += solve(&input[idx]);
	}

	auto answer = horizontal_sum(v);

	std::cout << answer << std::endl;

	return 0;
}

