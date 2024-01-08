const HOST = 'http://127.0.0.1:8000/';
const config = {
    'Access-Control-Allow-Origin': HOST,
    'Access-Control-Allow-Credentials': true,
    'Content-Type': 'text/plain',
};

import axios from 'axios'

export function get(path, params) {
    var instance = getInstance();
    return instance.get(HOST + path, params, config);
}

export function post(path, params) {
    var instance = getInstance();
    return instance.post(HOST + path, params, config);
}

export function put(path, params) {
    var instance = getInstance();
    return instance.put(HOST + path, params, config);
}

export function del(path, params) {
    var instance = getInstance();
    return instance.delete(HOST + path, params, config);
}
    
function getInstance() {
    return axios.create({
        withCredentials: true,
        credentials: 'include',
        sameSite: 'none',
    });
}