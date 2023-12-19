const HOST = 'http://127.0.0.1:8000/';

import axios from 'axios'

export function get(path, params) {
    axios.create({
        baseURL: HOST + path,
        withCredentials: true,
        sameSite: 'none',
    });
    return axios.get(HOST + path, params,
        {
            'Access-Control-Allow-Origin': HOST,
            'Access-Control-Allow-Credentials': true,
            'Content-Type': 'text/plain',
            
        });
}
