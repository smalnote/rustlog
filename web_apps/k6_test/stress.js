import http from 'k6/http';

export const options = {
    stages: [
        { duration: '1m', target: 200 },
        { duration: '3m', target: 200 },
        { duration: '1m', target: 0 },
    ]
};

export default() => {
    http.get('http://k8s0.devin.lan:30300/devices');
};
