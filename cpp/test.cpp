#include "m.hpp"
#include <assert.h>
#include <string>

void test_sum() { assert(55 == sum(10)); }

void test_triangle() { assert(15 == sum(5)); }

int main() {
  test_sum();
  test_triangle();
}
