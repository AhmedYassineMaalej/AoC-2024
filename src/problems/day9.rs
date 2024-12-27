#[derive(Debug)]
enum DiskEntry {
    FreeSpace { size: usize },
    File { id: usize, size: usize },
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let mut memory = Vec::new();

    for (idx, size) in input.trim().char_indices() {
        let size = size.to_digit(10).unwrap() as usize;
        if idx % 2 == 0 {
            let id = idx / 2;
            memory.push(DiskEntry::File { id, size });
        } else {
            memory.push(DiskEntry::FreeSpace { size });
        }
    }

    let mut clean_memory: Vec<DiskEntry> = Vec::new();

    let mut write_idx = 0;
    while write_idx < memory.len() {
        let block = &memory[write_idx];
        match *block {
            DiskEntry::File { id, size } => clean_memory.push(DiskEntry::File { id, size }),
            DiskEntry::FreeSpace { size: freesize } => {
                fill_freespace(&mut memory, freesize, write_idx, &mut clean_memory);
            }
        }
        write_idx += 1;
    }

    check_sum(&clean_memory)
}

fn fill_freespace(
    memory: &mut Vec<DiskEntry>,
    mut freesize: usize,
    write_idx: usize,
    clean_memory: &mut Vec<DiskEntry>,
) {
    let mut read_idx: usize = memory.len() - 1;

    while freesize > 0 && read_idx > write_idx {
        if let DiskEntry::File { id, size: filesize } = memory[read_idx] {
            if filesize <= freesize {
                clean_memory.push(DiskEntry::File { id, size: filesize });
                freesize -= filesize;
                memory.remove(read_idx);
                read_idx -= 1;
            } else {
                // filesize > freesize
                clean_memory.push(DiskEntry::File { id, size: freesize });
                memory[read_idx] = DiskEntry::File {
                    id,
                    size: filesize - freesize,
                };
                freesize = 0;
            }
        } else {
            read_idx -= 1;
        }
    }
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let mut memory = Vec::new();

    for (idx, size) in input.trim().char_indices() {
        let size = size.to_digit(10).unwrap() as usize;
        if idx % 2 == 0 {
            let id = idx / 2;
            memory.push(DiskEntry::File { id, size });
        } else {
            memory.push(DiskEntry::FreeSpace { size });
        }
    }

    let mut i = memory.len() - 1;
    while i > 0 {
        if let DiskEntry::File { id, size: filesize } = memory[i] {
            let mut insertion_idx = 0;

            loop {
                if let DiskEntry::FreeSpace { size: freesize } = memory[insertion_idx] {
                    if freesize > filesize {
                        memory[i] = DiskEntry::FreeSpace { size: filesize };
                        memory[insertion_idx] = DiskEntry::File { id, size: filesize };
                        memory.insert(
                            insertion_idx + 1,
                            DiskEntry::FreeSpace {
                                size: freesize - filesize,
                            },
                        );
                        break;
                    }

                    if freesize == filesize {
                        memory[i] = DiskEntry::FreeSpace { size: filesize };
                        memory[insertion_idx] = DiskEntry::File { id, size: filesize };
                        break;
                    }
                }

                if insertion_idx == i {
                    break;
                }

                insertion_idx += 1;
            }
        }
        i -= 1;
    }

    check_sum(&memory)
}

fn check_sum(memory: &[DiskEntry]) -> usize {
    let mut position = 0;
    let mut result = 0;

    for block in memory {
        match *block {
            DiskEntry::FreeSpace { size } => position += size,
            DiskEntry::File { id, size } => {
                for _ in 0..size {
                    result += id * position;
                    position += 1;
                }
            }
        }
    }

    result
}
