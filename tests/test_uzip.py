import uzip


# noinspection SpellCheckingInspection
def test_b2048_encode():
    assert uzip.b2048encode(b"Hello there!") == "ϓțƘ໐úɡਪϵà"
    assert uzip.b2048decode("ϓțƘ໐úɡਪϵà") == b"Hello there!"

