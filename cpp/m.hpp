unsigned int sum(unsigned int n) {
  unsigned int sum = 0;
  for (unsigned int k = 1; k <= n; ++k) {
    sum += k;
  }
  return sum;
}
