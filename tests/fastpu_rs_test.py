import pytest
import fastpy_rs

def test_double_basic():
    assert fastpy_rs.double(0) == 0
    assert fastpy_rs.double(1) == 2
    assert fastpy_rs.double(42) == 84

