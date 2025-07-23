import time 
from datetime import timedelta
from fastpy_rs import benchmark

# This defines how much overhead the call to the rust fn can have in Microseconds.  
MAX_OVERHEAD_IN_MICROS = 500

def test_benchmark_fn():

    function = lambda x: time.sleep(x)
    seconds = 5

    result: timedelta = benchmark.benchmark_fn(function, seconds)
    expected_microseconds = seconds * 1_000_000

    total_micros = result.total_seconds() * 1_000_000 
    dif_to_expected = total_micros - expected_microseconds
    print(dif_to_expected)

    assert dif_to_expected <= MAX_OVERHEAD_IN_MICROS and dif_to_expected > 0

def test_benchmark_fn_with_kwargs():
    ### A function that sleeps for one second per kwarg
    def kwarg_function(**kwargs):
        for arg in kwargs:
            time.sleep(1)
    
    result: timedelta = benchmark.benchmark_fn(kwarg_function, arg1="First argument", arg2="second argument")
    # One second for each kwarg
    expected_microseconds = 2 * 1_000_000
    
    total_micros = result.total_seconds() * 1_000_000 
    dif_to_expected = total_micros - expected_microseconds
    print(dif_to_expected)
    assert dif_to_expected <= MAX_OVERHEAD_IN_MICROS and dif_to_expected > 0

def test_benchmark_fn_with_multiple_args():
    def multiple_args_function(positioned_arg, another_positioned_arg, kwarg = "Default"): 
        time.sleep(3)

    useless_var = ""

    result: timedelta = benchmark.benchmark_fn(multiple_args_function, useless_var, useless_var, kwarg="Not Default")
    
    # One second for each kwarg
    expected_microseconds = 3 * 1_000_000
    total_micros = result.total_seconds() * 1_000_000 
    dif_to_expected = total_micros - expected_microseconds
    print(dif_to_expected)
    assert dif_to_expected <= MAX_OVERHEAD_IN_MICROS and dif_to_expected > 0

def test_custom_callable():
    class CustomCallable: 
        def __call__(self, *args, **kwds):
            time.sleep(kwds["sleeptime"])

    
    custom_callable = CustomCallable()
    time_to_sleep = 1
    result = benchmark.benchmark_fn(custom_callable, sleeptime=time_to_sleep)

    expected_microseconds = time_to_sleep * 1_000_000
    total_micros = result.total_seconds() * 1_000_000 
    dif_to_expected = total_micros - expected_microseconds
    print(dif_to_expected)
    assert dif_to_expected <= MAX_OVERHEAD_IN_MICROS and dif_to_expected > 0

def test_invalid_args():
    def func():
        ...
    try:
        # Expect: TypeError due to invalid arguments.
        benchmark.benchmark_fn(func, 1, arg="wtf")
        assert False
    except Exception as e:
        assert isinstance(e, TypeError)

def test_non_callable():
    try:
        # Expect: TypeError due to attept to call a non callable
        benchmark.benchmark_fn(1, 1)
        assert False
    except Exception as e:
        assert isinstance(e, TypeError)
