from uzip import uzip


# noinspection SpellCheckingInspection
def test_compress():
    assert uzip.compress(b"Hello there!") == "ϓțƘ໐úɡਪϵà"
