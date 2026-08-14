from rust_neotools import Php5MtRandom, Php5Random


def test_php5_random_seed_one() -> None:
    r = Php5Random(1)
    values = [r.rand() for _ in range(5)]
    assert values == [1804289383, 846930886, 1681692777, 1714636915, 1957747793]


def test_php5_random_seed_one_range() -> None:
    r = Php5Random(1)
    values = [r.rand_range(0, 100) for _ in range(5)]
    assert values == [84, 39, 79, 80, 92]


def test_php5_mt_random_seed_one() -> None:
    r = Php5MtRandom(1)
    values = [r.rand() for _ in range(5)]
    assert values == [1244335972, 15217923, 1546885062, 2002651684, 2135443977]


def test_php5_mt_random_seed_one_range() -> None:
    r = Php5MtRandom(1)
    values = [r.rand_range(0, 100) for _ in range(5)]
    assert values == [58, 0, 72, 94, 100]


def test_php5_mt_random_seed_forty_two_range() -> None:
    # Verified against a real php5.6-cli binary:
    # mt_srand(42); mt_rand(0,23).",".mt_rand(0,59) === "15,47"
    r = Php5MtRandom(42)
    assert r.rand_range(0, 23) == 15
    assert r.rand_range(0, 59) == 47
