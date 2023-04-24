# -*- coding: utf-8 -*-
u"Bitonic Merge Sort Module"

def sort(x, up):

    if len(x) <= 1:
        return x
    else:
        # 前半は昇順、後半は降順にソート
        mid_point = len(x) // 2
        first = sort(x[:mid_point], True)
        second = sort(x[mid_point:], False)
        
        # これで一旦バイトニック列ができたはず。
        
        x1 = first + second
        return _sub_sort(x1, up)
    
def _sub_sort(x, up):

    if len(x) == 1:
        return x
    else:
        _compare_and_swap(x, up)
        
    mid_point = len(x) // 2
    first = _sub_sort(x[:mid_point], up)
    second = _sub_sort(x[mid_point:], up)
    return first + second

def _compare_and_swap(x, up):

    mid_point = len(x) // 2
    for i in range(mid_point):
        if (x[i] > x[mid_point + i]) == up:
            x[i], x[mid_point + i] = x[mid_point + i], x[i]