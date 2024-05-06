type AnyFunction = (...args: any[]) => any;

export function throttle<T extends AnyFunction>(cb: T, delay: number = 1000) {
  let should_wait = false;
  let waiting_args: Parameters<T> | null = null;

  const timeout_func = () => {
    if (waiting_args === null) {
      should_wait = false;
      return;
    }
    cb(...waiting_args);
    waiting_args = null;
    setTimeout(timeout_func, delay);
  };

  return (...args: Parameters<T>) => {
    if (should_wait) {
      waiting_args = args;
      return;
    }
    cb(...args);
    should_wait = true;
    setTimeout(timeout_func, delay);
  };
}
