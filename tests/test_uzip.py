import uzip


# noinspection SpellCheckingInspection
def test_b2048_encode():
    assert uzip.b2048encode(b"Hello there!") == "ϓțƘ໐úɡਪϵà"
    assert uzip.b2048decode("ϓțƘ໐úɡਪϵà") == b"Hello there!"


def test_b65536_encode():
    sample = "かわいい".encode('utf-8')
    assert uzip.b65536encode(sample) == "𠟣𦦋𡖂𠟣𦦄𠪁"
    assert uzip.b65536decode("𠟣𦦋𡖂𠟣𦦄𠪁") == sample

