from typing import List

class Chunk:
    pass

class Equal(Chunk):
    def __init__(self, s: str): ...

class Delete(Chunk):
    def __init__(self, s: str): ...

class Insert(Chunk):
    def __init__(self, s: str): ...

def diff(a: str, b: str) -> List[Chunk]: ...
