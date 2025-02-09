import React, { useEffect, useRef } from 'react';
import videojs from 'video.js';

const Home = () => {
    const videoRef = useRef(null);

    useEffect(() => {
        if (videoRef.current) {
            const player = videojs(videoRef.current, {
                controls: true,
                autoplay: true,
                fluid: true,
                sources: [{ src: "rtmp://localhost/live/ps5", type: "rtmp/mp4" }]
            });
            return () => player.dispose();
        }
    }, []);

    return (
        <div>
            <h1>PS5 Streaming</h1>
            <video ref={videoRef} className="video-js vjs-default-skin" />
        </div>
    );
};

export default Home;
