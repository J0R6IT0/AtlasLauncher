import React, { useState } from 'react';
import '../styles/Library.css';

function Library(): JSX.Element {
    return (
        <div className='library'>
            <div className='instance'>
                <img className='instance-background' src="https://cdn.modrinth.com/data/BYfVnHa7/00a1b981ab6eb08b67bf9d9d9c910f3c404cfb56.png" alt="" />
                <div className='instance-icon-container'>
                    <img className='instance-icon' src="https://cdn.modrinth.com/data/BYfVnHa7/00a1b981ab6eb08b67bf9d9d9c910f3c404cfb56.png" alt="" />
                </div>
                <div className='instance-content'>
                    <span className='instance-name'>Fabulously Optimized</span>
                </div>
            </div>
            <div className='instance'>
                <img className='instance-background' src="https://cdn.modrinth.com/data/paoFU4Vl/bfdc40da7ccbac30899fc89f0a6b9f524c09663c.png" alt="" />
                <div className='instance-icon-container'>
                    <img className='instance-icon' src="https://cdn.modrinth.com/data/paoFU4Vl/bfdc40da7ccbac30899fc89f0a6b9f524c09663c.png" alt="" />
                </div>
                <div className='instance-content'>
                    <span className='instance-name'>Additive</span>
                </div>
            </div>
            <div className='instance'>
                <img className='instance-background' src="https://cdn.modrinth.com/data/JYB6M4ar/5f0f9f5f9cd3a7a2f9d7150e722d15c94125e591.gif" alt="" />
                <div className='instance-icon-container'>
                    <img className='instance-icon' src="https://cdn.modrinth.com/data/JYB6M4ar/5f0f9f5f9cd3a7a2f9d7150e722d15c94125e591.gif" alt="" />
                </div>
                <div className='instance-content'>
                    <span className='instance-name'>Yoru</span>
                </div>
            </div>
            
        </div>
    );
}

export default Library;
