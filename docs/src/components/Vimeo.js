import React from 'react';

export default function Vimeo(props) {
  let vimeoUrl = 'https://player.vimeo.com/video/' + props.vimeoId + '?muted=1&autopause=0&autoplay=1&loop=1&title=0&byline=0&portrait=0';
  return (
    <div style={{
      padding: '56.23% 0 0 0', 
      position: 'relative'
    }}>
      <iframe 
        src={vimeoUrl}
        style={{
          position: 'absolute', 
          top: 0, 
          left: 0, 
          width: '100%', 
          height: '100%'
        }}
        frameBorder='0'
        autoPlay
        allowFullScreen>
      </iframe>
    </div>
  );
}

