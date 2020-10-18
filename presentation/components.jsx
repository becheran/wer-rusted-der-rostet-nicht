import React from 'react';

export const Website = ({ name, url }) => {
    return (
        <>
            <h2><a href={url}>{name}</a></h2>
            <iframe title={name} style={{ width: "80%", height: "70%" }} src={url}></iframe>
        </>);
}
