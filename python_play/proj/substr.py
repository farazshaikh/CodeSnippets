def substr_rec(hay: str, needle: str) -> bool:
    if len(needle) == 0:
        return True
    if len(needle) > len(hay):
        return False
    
    # needle is less than equal to hay
    if needle[0] == hay[0]:
        return substr_rec(hay[1:], needle[1:])  
    else:
        return substr_rec(hay[1:], needle)


# faraz
#   raz 
def substr_no_rec(hay: str, needle: str)-> bool:
    for i in (range(0, len(hay) - len(needle) + 1)):
        for j in range(0, len(needle)):
            if needle[j] != hay[i]:
                break
        else:
            return True
    return False



def substr_h_kmp(hay: str, needle: str)-> (bool, int, int):
    assert len(needle) >= len(hay)
    for i in (range(0, len(hay) - len(needle) + 1)):
        for j in range(0, len(needle)):
            if needle[j] != hay[i]:
                break
        else:
            return True, 0,0 
    return False, i, j


def lps(needle: str) -> list[int]:
    lps = [0] * len(needle)
    l = 0
    i = 1
    while i < len(needle):
        if needle[i] == needle[l]:
            lps[i] = l + 1
            l += 1
            i += 1
        else:
            if l != 0:
                l = lps[l-1]
            else:
                lps[i] = 0
                i += 1
    return lps

def flps(needle: str) -> list[int]:
    '''
    Preprocess the needle to build the least prefix suffix array

    Args:
        needle (str): the needle to preprocess  

    Returns: 
        list[int]: the lps array where lps[i] is the length prefix of the array that is also a suffix of the array
    '''
    lps = [0] * len(needle)
    if len(needle) == 0:
        return lps
    
    lps[0] = 0
    for i in range(1, len(needle)):
        if needle[i] == needle[lps[i-1]]:
            lps[i] = lps[i-1] + 1
        else:
            lps[i] = 0

    return lps

# knuth morris pratt
def substr_kmp(hay: str, needle: str)-> bool:
    if needle == "":
        return True
    if len(needle) > len(hay):  
        return False
    # build the lps array
    lps = flps(needle)

    probe = 0
    while probe  < len(hay) - len(needle) + 1:
        for i in range(0, len(needle)):
            if needle[i] != hay[probe + i]:
                break
        else:
            return True
        
        if i > 0:
            probe = probe + i - lps[i-1]
        else:
            probe += 1   
        print(probe)
    return False     


print(substr_rec("hello", "fo"))
print(substr_no_rec("hello", "ll"))