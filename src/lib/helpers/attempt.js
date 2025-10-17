export function attempt(fn) {
    try {
        return fn();
    } catch (error) {
        console.error(error);
        return null;
    }
}
