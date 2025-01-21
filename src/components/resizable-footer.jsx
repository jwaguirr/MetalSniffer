import React, { useState } from 'react';
import { ResizableBox } from 'react-resizable';
import 'react-resizable/css/styles.css'; // Import styles for resizing handles

const ResizableFooter = ({packetData}) => {
  const [footerHeight, setFooterHeight] = useState(200); // Initial height
  const snapPoints = [10, 50, 100, 200, 300, 400, 500];
  const [openLayer2, setOpenLayer2] = useState(false);
  const [openLayer3, setOpenLayer3] = useState(false);
  const [openLayer4, setOpenLayer4] = useState(false);
  // Find the closest snap point
  const getClosestSnapPoint = (height) => {
    return snapPoints.reduce((prev, curr) =>
      Math.abs(curr - height) < Math.abs(prev - height) ? curr : prev
    );
  };


  return  (
    <div
      className="fixed bottom-0 left-0 right-0 bg-white border-t-2 border-black"
    >
      {/* Use ResizableBox to make the footer resizable */}
      <ResizableBox
        height={footerHeight}
        width={Infinity} // Keep the width fixed
        axis="y" // Only allow vertical resizing
        resizeHandles={['n']} // Resizing handle at the top (north)
        minConstraints={[Infinity, snapPoints[0]]} // Minimum height (first snap point)
        maxConstraints={[Infinity, snapPoints[snapPoints.length - 1]]} // Maximum height (last snap point)
        onResizeStop={(e, { size }) => {
          const snappedHeight = getClosestSnapPoint(size.height);
          setFooterHeight(snappedHeight);
        }}
      >
        <div
          style={{
            height: `${footerHeight}px`,
          }}
          className="h-full overflow-y-auto"
        >
          {/* Main content of the footer */}
          {packetData ? (
                <div className="p-2 pt-4 h-full">
                    <h1 className='text-center font-bold'>Packet Information</h1>
                <div className="flex flex-col w-full max-h-96 mt-4 space-y-4">
                    <div className='flex flex-col'>
                        <div className='flex flex-row'>
                            Level 3 <span onClick={() => setOpenLayer2(!openLayer2)} className={`cursor-pointer select-none ml-3 ${openLayer2 && "rotate-90"} duration-200`}>&gt;</span>
                        </div>
                        {openLayer2 && (
                            Object.entries(packetData["Layer 1"]).map(([key, value]) => {
                                return (
                                    <div className="flex flex-row" key={key}>
                                        <div className="mr-2 font-bold">{key}:</div>
                                        <div>{value}</div>
                                    </div>
                                );
                            })
                        )}
                    </div>
                    <div className='flex flex-col'>
                        <div className='flex flex-row'>
                            Level 4 (Transport) <span onClick={() => setOpenLayer3(!openLayer3)} className={`cursor-pointer select-none ml-3 ${openLayer3 && "rotate-90"} duration-200`}>&gt;</span>
                        </div>
                        {openLayer3 && (
                            Object.entries(packetData["Layer 2"]).map(([key, value]) => {
                                return (
                                    <div className="flex flex-row" key={key}>
                                        <div className="mr-2 font-bold">{key}:</div>
                                        <div>{value}</div>
                                    </div>
                                );
                            })
                        )}
                    </div>                
                    <div className='flex flex-col'>
                        <div className='flex flex-row'>
                            Level 5-7 (Protocol)<span onClick={() => setOpenLayer4(!openLayer4)} className={`cursor-pointer select-none ml-3 ${openLayer4 && "rotate-90"} duration-200`}>&gt;</span>
                        </div>
                        {openLayer4 && (
                            Object.entries(packetData["Layer 3"]).map(([key, value]) => {
                                return (
                                    <div className="flex flex-row" key={key}>
                                        <div className="mr-2 font-bold">{key}:</div>
                                        <div>{value}</div>
                                    </div>
                                );
                            })
                        )}
                    </div>   
                </div>
                </div>
          ) : (
            <div>
                Please select a packet!
            </div>
          )}
                          </div>
                          </ResizableBox>
                </div>


  );
};

export default ResizableFooter;
