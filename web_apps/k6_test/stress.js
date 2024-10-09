import http from 'k6/http';

export const options = {
    stages: [
        { duration: '2m', target: 50 },
        { duration: '6m', target: 50 },
        { duration: '2m', target: 0 },
    ]
};

export default() => {
    http.get('http://k8s0.devin.lan:30300/devices');
};
