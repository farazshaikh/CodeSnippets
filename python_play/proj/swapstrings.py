import sys
def find_all_occurences(input: str, sub: str) -> list:
    start = 0
    l = len(sub)
    occurences = []
    while True:
        start = input.find(sub, start)
        if start == -1:
            break
        occurences.append(start)
        start += l
    return occurences

from typing import List, Tuple

def word_index(input: str, first: str, second: str) -> Tuple[List[str], List[int], List[int]]:
    words  = input.split()
    first_index, second_index = [], []
    for i, word in enumerate(words):
        if word == first: 
            first_index.append(i)
        elif word == second:
            second_index.append(i) 
    return words, first_index, second_index 

def swap_strings(ip: str, first: str, second:str) -> str:
    # find all occurance of string
    wordlist, first_index, second_index = word_index(ip, first, second)
    for pair in zip(first_index, second_index):
        wordlist[pair[0]], wordlist[pair[1]] = wordlist[pair[1]], wordlist[pair[0]]

    return " ".join(wordlist)
    
    
if __name__ == "__main__":
   print(swap_strings(sys.argv[1], sys.argv[2], sys.argv[3]))  # python swapstrings.py "I am a good