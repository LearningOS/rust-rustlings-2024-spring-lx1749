/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


fn sort<T>(array: &mut [T])
where 
    T: std::cmp::PartialOrd + std::marker::Copy
{
	if array.len() == 1 {
        return 
    }
    let mid = array.len() / 2;
    sort(&mut array[..mid]);
    sort(&mut array[mid..]);
    let mut temp: Vec<T> = Vec::new();
    let mut i = 0;
    let mut j = mid;
    while i < mid && j < array.len(){
        if array[i] < array[j] {
            temp.push(array[i]);
            i += 1;
        } else {
            temp.push(array[j]);
            j += 1;
        }
    }
    while i < mid {
        temp.push(array[i]);
        i += 1;
    }
    while j < array.len() {
        temp.push(array[j]);
            j += 1;
    }
    for i in 0..array.len() {
        array[i] = temp[i];
    }
    
    
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}