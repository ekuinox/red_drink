import { useEffect } from 'react';

export const useDidMound = (func: () => void): void =>
  useEffect(() => {
    func();
  }, []);

export const useDidUnmount = (func: () => void): void =>
  useEffect(() => func, []);
