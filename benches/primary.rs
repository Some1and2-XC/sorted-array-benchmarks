
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use sorted_arrays::SortedArray;

const AMNT_OF_SAMPLES: i32 = 1;
const MAX_SAMPLE: i32 = 100000;

fn benchmark_two_finger(c: &mut Criterion) {

    let sizes = (1..=AMNT_OF_SAMPLES).map(|v| (v * MAX_SAMPLE) / AMNT_OF_SAMPLES).collect::<Vec<i32>>();

    let mut group = c.benchmark_group("two_finger");

    for size in sizes {

        group.throughput(Throughput::Elements(size.try_into().unwrap()));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &s| {
            b.iter(|| get_diff_o_n_squared(
                black_box(&mut SortedArray::<i32>::new_random(s, 0, s)),
                black_box(&mut SortedArray::<i32>::new_random(s, 0, s)),
            ));
        });
    }

    group.finish();

}


fn benchmark_intuitive(c: &mut Criterion) {

    let sizes = (1..=AMNT_OF_SAMPLES).map(|v| (v * MAX_SAMPLE) / AMNT_OF_SAMPLES).collect::<Vec<i32>>();

    let mut group = c.benchmark_group("intuitive");

    for size in sizes {

        group.throughput(Throughput::Elements(size.try_into().unwrap()));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &s| {
            b.iter(|| get_diff_o_n(
                black_box(&mut SortedArray::<i32>::new_random(s, 0, s)),
                black_box(&mut SortedArray::<i32>::new_random(s, 0, s)),
            ));
        });

    }

    group.finish();


}

fn benchmark_two_fors(c: &mut Criterion) {

    let sizes = (1..=AMNT_OF_SAMPLES).map(|v| (v * MAX_SAMPLE) / AMNT_OF_SAMPLES).collect::<Vec<i32>>();

    let mut group = c.benchmark_group("two_fors");

    for size in sizes {

        group.throughput(Throughput::Elements(size.try_into().unwrap()));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &s| {
            b.iter(|| get_diff_with_two_fors(
                black_box(&mut SortedArray::<i32>::new_random(s, 0, s)),
                black_box(&mut SortedArray::<i32>::new_random(s, 0, s)),
            ));
        });

    }

    group.finish();


}


criterion_group!(benches, benchmark_two_finger, benchmark_intuitive, benchmark_two_fors);
criterion_main!(benches);

fn get_diff_o_n<T: PartialOrd + Clone>(arr1: &mut SortedArray<T>, arr2: &mut SortedArray<T>) -> SortedArray<T> {

    let mut out_arr = SortedArray::<T>::new();

    while arr1.len() > 0 || arr2.len() > 0 {

        if arr1.len() == 0 {

            // Add the rest of the items back to the thing
            for i in 0..arr2.len() {

                let new_value = &arr2.get_inner()[i];
                if out_arr.contains(new_value).is_none() {
                    out_arr.add(arr2.get(i).clone());
                }
            }

            break;
        }


        if arr2.len() == 0 {
            for i in 0..arr1.len() {

                let new_value = &arr1.get(i);
                if out_arr.contains(new_value).is_none() {
                    out_arr.add(arr1.get(i).clone());
                }
            }

            break;
        }

        let val1 = arr1.get(0);
        let val2 = arr2.get(0);

        if val1 == val2 {

            // The choice for checking val1 is arbitrary (can be either)
            if out_arr.len() != 0 {
                if out_arr.get(out_arr.len() - 1) == val1 {
                    out_arr.remove(out_arr.len() - 1);
                }
            }

            arr1.remove(0);
            arr2.remove(0);

        }
        else if val1 < val2 {

            out_arr.add(val1.clone());
            arr1.remove(0);

        }
        // val1 > val2
        else {
            out_arr.add(val2.clone());
            arr2.remove(0);
        }

    }

    return out_arr;

}

fn get_diff_o_n_squared<T: PartialOrd + Clone>(arr1: &mut SortedArray<T>, arr2: &mut SortedArray<T>) -> SortedArray<T> {

    let mut out_arr = SortedArray::<T>::new();

    for i in 0..arr1.len() {
        let v = arr1.get(i);
        // Add the value if it isn't found on the other one
        if arr2.contains(v).is_none() {
            out_arr.add(v.clone());
        }
    }

    for i in 1..arr1.len() {
        let v = arr2.get(i);
        // Add the value if it isn't found on the other one
        if arr1.contains(v).is_none() {
            out_arr.add(v.clone());
        }
    }

    return out_arr;

}

fn get_diff_with_two_fors<T: PartialOrd + Clone>(arr1: &mut SortedArray<T>, arr2: &mut SortedArray<T>) -> SortedArray<T> {

    let mut out_arr = SortedArray::<T>::new();

    for i in 0..arr1.len() {
        let v = arr1.get(i);

        // Add the value if it isn't found on the other one
        for j in 0..arr2.len() {
            if v == arr2.get(j) {
                out_arr.add(v.clone());
            }
        }
    }

    for i in 0..arr2.len() {
        let v = arr2.get(i);
        // Add the value if it isn't found on the other one
        for j in 0..arr1.len() {
            if v == arr1.get(j) {
                out_arr.add(v.clone());
            }
        }
    }

    return out_arr;


}
