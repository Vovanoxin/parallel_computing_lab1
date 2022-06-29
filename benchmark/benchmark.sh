#!/bin/bash

for matr_size in $(cat seq_tests_data)
do
  for i in {1..10}; do
    ../target/debug/sequential -n "$matr_size" -m "$matr_size" --time -k 10
  done | awk 'BEGIN{FS="\t"}
  {alloc = alloc + $1; a_n++; fill = fill + $2; f_n++; func_t = func_t + $3; func_n++}
  END{printf("%f\t%f\t%f\n", alloc/a_n, fill/f_n, func_t/func_n)}'
done > bench_results

for threads_num in $(cat threads_data)
do
  for matr_size in $(cat seq_tests_data)
  do
    for i in {1..10}; do
      ../target/debug/parallel -n "$matr_size" -m "$matr_size" --time -k 10 --threads $threads_num
    done | awk 'BEGIN{FS="\t"}
    {alloc = alloc + $1; a_n++; fill = fill + $2; f_n++; func_t = func_t + $3; func_n++}
    END{printf("%f\t%f\t%f\n", alloc/a_n, fill/f_n, func_t/func_n)}'
  done > "parallel_results_$threads_num"
done

for matr_size in $(cat seq_tests_data)
do
  for i in {1..10}; do
    ../target/release/parallel -n "$matr_size" -m "$matr_size" --time -k 10 --threads 8
  done | awk 'BEGIN{FS="\t"}
  {alloc = alloc + $1; a_n++; fill = fill + $2; f_n++; func_t = func_t + $3; func_n++}
  END{printf("%f\t%f\t%f\n", alloc/a_n, fill/f_n, func_t/func_n)}'
done > "parallel_8_optimized"

for matr_size in $(cat seq_tests_data)
do
  for i in {1..10}; do
    ../target/release/sequential -n "$matr_size" -m "$matr_size" --time -k 10
  done | awk 'BEGIN{FS="\t"}
  {alloc = alloc + $1; a_n++; fill = fill + $2; f_n++; func_t = func_t + $3; func_n++}
  END{printf("%f\t%f\t%f\n", alloc/a_n, fill/f_n, func_t/func_n)}'
done > "seq_optimized"

python plot_graph.py