use std::collections::VecDeque;

use itertools::Itertools;
use rs_2024_09::*;

fn main() {
    let input = rs_2024_09::static_read("input1.txt");
    let (mut files, mut free) = parse(input);
    let mut output = Vec::new();
    let mut fill_space = false;
    loop {
        if fill_space {
            let Some(file) = files.pop_back() else {
                break;
            };
            let Some(free_space) = free.pop_front() else {
                output.push(file);
                break;
            };
            if free_space == file.size {
                output.push(file);
                fill_space = false;
                continue;
            }
            if free_space > file.size {
                output.push(File {
                    id: file.id,
                    size: file.size,
                });
                free.push_front(free_space - file.size);
                continue;
            }
            files.push_back(File {
                id: file.id,
                size: file.size - free_space,
            });
            output.push(File {
                id: file.id,
                size: free_space,
            });
            fill_space = false;
        }
        
        let Some(file) = files.pop_front() else {
            break;
        };
        output.push(file);
        let Some(file) = files.pop_back() else {
            break;
        };
        let Some(free_space) = free.pop_front() else {
            output.push(file);
            break; // handle correctly
        };
        if free_space == file.size {
            output.push(file);
            continue;
        }
        if free_space < file.size {
            output.push(File {
                id: file.id,
                size: free_space,
            });
            files.push_back(File {
                id: file.id,
                size: file.size - free_space,
            });
            continue;
        }
        free.push_front(free_space - file.size);
        output.push(file);
        fill_space = true;
    }
    let result: usize = calculate_checksum(output);
    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_09::static_read("example1.txt");
        let (mut files, mut free) = parse(input);
        let mut output = Vec::new();
        let mut fill_space = false;
        loop {
            if fill_space {
                let Some(file) = files.pop_back() else {
                    break;
                };
                let Some(free_space) = free.pop_front() else {
                    output.push(file);
                    break;
                };
                if free_space == file.size {
                    output.push(file);
                    continue;
                }
                if free_space > file.size {
                    output.push(File {
                        id: file.id,
                        size: file.size,
                    });
                    free.push_front(free_space - file.size);
                    continue;
                }
                files.push_back(File {
                    id: file.id,
                    size: file.size - free_space,
                });
                output.push(File {
                    id: file.id,
                    size: free_space,
                });
                fill_space = false;
            }
            let Some(file) = files.pop_front() else {
                break;
            };
            output.push(file);
            let Some(file) = files.pop_back() else {
                break;
            };
            let Some(free_space) = free.pop_front() else {
                output.push(file);
                break; // handle correctly
            };
            if free_space == file.size {
                output.push(file);
                continue;
            }
            if free_space < file.size {
                output.push(File {
                    id: file.id,
                    size: free_space,
                });
                files.push_back(File {
                    id: file.id,
                    size: file.size - free_space,
                });
                continue;
            }
            free.push_front(free_space - file.size);
            fill_space = true;
            output.push(file);
        }
        let result: usize = calculate_checksum(output);
        println!("{}", result);
        assert!(result == 1928);
    }
}

fn calculate_checksum(files: Vec<File>) -> usize {
    let mut checksum = 0;
    let mut idx = 0;
    for file in files {
        for _ in 0..file.size {
            checksum += file.id * idx;
            idx += 1;
        }
    }
    checksum
}

struct File {
    id: usize,
    size: usize,
}

fn parse(input: &str) -> (VecDeque<File>, VecDeque<usize>) {
    input.chars().filter(|c|char::is_numeric(*c)).enumerate().fold(
        (VecDeque::new(), VecDeque::new()),
        |(mut files, mut free): (VecDeque<File>, VecDeque<usize>), (idx, size)| {
            if idx % 2 == 0 {
                files.push_back(File {
                    id: files.len(),
                    size: size.to_string().parse().unwrap(),
                });
            } else {
                free.push_back(size.to_string().parse().unwrap());
            }
            (files, free)
        },
    )
}
