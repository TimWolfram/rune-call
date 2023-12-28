const HOST = 'http://127.0.0.1:8000/';

import axios from 'axios'

export function get(path, params) {
    var instance = axios.create({
        withCredentials: true,
        credentials: 'include',
        sameSite: 'none',
        
    });
    return instance.get(HOST + path, params,
        {
            'Access-Control-Allow-Origin': HOST,
            'Access-Control-Allow-Credentials': true,
            'Content-Type': 'text/plain',
            
        });
}
