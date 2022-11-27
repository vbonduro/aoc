windows = [0] * 3
n_increases = 0
start_windex = 0
end_windex = 0
prev_window_sum = 0
print(windows)
with open('input') as readings:
    windows[0] = int(readings.readline())
    for reading in readings:
        end_windex = (end_windex + 1) % 3
        i = start_windex
        windows[end_windex] = int(reading)
        count = 1
        while i != end_windex:
            windows[i] += int(reading)
            i = (i + 1) % 3
            count += 1
        if count == 3:
            window_sum = windows[start_windex]
            if prev_window_sum > 0 and prev_window_sum < window_sum:
                n_increases += 1
            prev_window_sum = window_sum
            start_windex = (start_windex + 1) % 3
print(n_increases)
