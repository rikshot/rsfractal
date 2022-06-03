import init, { child_entry_point } from './rsfractal_wasm.js'

self.onmessage = async event => {
    try {
        await init(event.data[0], event.data[1]);
    } catch (err) {
        setTimeout(() => {
            throw err;
        });
        throw err;
    }

    self.onmessage = async event => {
        child_entry_point(event.data);
    };
};