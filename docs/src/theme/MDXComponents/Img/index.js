import Vimeo from '@site/src/components/Vimeo';
import clsx from 'clsx';
import React from 'react';
import styles from './styles.module.css';

function transformImgClassName(className) {
  return clsx(className, styles.img);
}
export default function MDXImg(props) {
  if ((props.src.endsWith(".gif") || props.src.endsWith(".mp4")) && (typeof props.title !== 'undefined')) {
    return <Vimeo vimeoId={props.title} />;
  };
  return (
    <img
      loading="lazy"
      {...props}
      className={transformImgClassName(props.className)}
    />
  );
}