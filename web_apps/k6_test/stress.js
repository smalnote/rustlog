import http from 'k6/http';

export const options = {
    stages: [
        { duration: '5m', target: 200 },
        { duration: '50m', target: 200 },
        { duration: '5m', target: 0 },
    ]
};

export default() => {
    http.get('http://k8s0.devin.lan:30300/devices');
};
