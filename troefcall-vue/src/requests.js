const HOST = 'http://127.0.0.1:8000/';
const headers = {
    'Cache-Control': 'no-cache',
    'Content-Type': 'application/json',
};

import axios from 'axios'

export function get(path) {
    var instance = getInstance();
    return instance.get(HOST + path, headers);
}

export function post(path, params) {
    var instance = getInstance();
    return instance.post(HOST + path, params, headers);
}

export function put(path, params) {
    var instance = getInstance();
    return instance.put(HOST + path, params, headers);
}

export function del(path, params) {
    var instance = getInstance();
    return instance.delete(HOST + path, params, headers);
}
    
function getInstance() {
    return axios.create({
        withCredentials: true,
        credentials: 'include',
        sameSite: 'none',
        headers: headers
    });
}