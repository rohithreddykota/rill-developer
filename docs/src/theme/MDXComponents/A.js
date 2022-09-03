import React from 'react';
import Link from '@docusaurus/Link';
import Vimeo from '@site/src/components/Vimeo';

export default function MDXA(props) {
  if ((props.href.endsWith(".gif") || props.href.endsWith(".mp4")) && (typeof props.title !== 'undefined')) {
    return (
      <Vimeo vimeoId={props.title} />
    )
  };
  return <Link {...props} />
}
