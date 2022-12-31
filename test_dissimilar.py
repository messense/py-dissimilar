import dissimilar
from dissimilar import Equal, Delete, Insert


def test_dissimilar():
    a = '[乀丁abcd一]'
    b = '[一abcd丁]'
    chunks = dissimilar.diff(a, b)
    assert chunks == [
       Equal("["),
       Delete("乀丁"),
       Insert("一"),
       Equal("abcd"),
       Delete("一"),
       Insert("丁"),
       Equal("]"),
    ]
