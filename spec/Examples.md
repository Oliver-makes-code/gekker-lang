# Examples

Here are a couple code examples that may or may not work

## Bubble sort

```
where
    T : Compare
func BubbleSort(arr: ref mut [T]) {
    for i in 0..(arr.Size - 1) {
        mut swapped = false;
        for j in 0..(arr.Size - i - 1) {
            if arr[j] > arr[j+1] {
                let tmp = arr[j];
                arr[j] = arr[j+1];
                arr[j+1] = tmp;
                swapped = true;
            }
        }
        break if !swapped;
    }
}
```
