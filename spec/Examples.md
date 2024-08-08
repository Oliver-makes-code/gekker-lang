# Examples

Here are a couple code examples that may or may not work

## Bubble sort

```
where
    T : Compare<T>;
func BubbleSort(arr: ref mut [T]) {
    for i in 0..(arr.Size - 1) {
        mut swapped = false;
        for j in 0..(arr.Size - i - 1) {
            if arr[j] > arr[j+1] {
                [arr[j], arr[j+1]] = [arr[j+1], arr[j]];
                swapped = true;
            }
        }
        break if !swapped;
    }
}
```

```
#[Flags]
enum WindowFlags : u32 {
    Fullscreen = 0x00000001,
    OpenGl     = 0x00000002,
    Shown      = 0x00000004,
    Hidden     = 0x00000008,
    Borderless = 0x00000010,
    Resizable  = 0x00000020,
    //...
}
```
