unsigned int sum(unsigned int n) {
  unsigned int sum = 0;
  for (unsigned int k = 1; k <= n; ++k) {
    sum += k;
  }
  return sum;
}

unsigned int triangle(unsigned int n) {
  unsigned int res = 0;
  for (unsigned int k = 0; k < n; ++k) {
    res += k + 1;
  }
  return res;
}
