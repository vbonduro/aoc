n_increases = 0
with open('input') as readings:
    prev_reading = readings.readline()
    for reading in readings:
        if int(reading) > int(prev_reading):
            n_increases += 1
        prev_reading = reading
print(n_increases)
