'use client'

import React, { useState } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import DeckGL from '@deck.gl/react';
import { TileLayer } from '@deck.gl/geo-layers';
import { BitmapLayer, IconLayer, LineLayer } from '@deck.gl/layers';
import { MapViewState, PickingInfo } from '@deck.gl/core';
import { Icon } from '@mui/material';
import PlaceIcon from '@mui/icons-material/Place';


const INITIAL_VIEW_STATE: MapViewState = {
  longitude: -2.5, // Centered on the UK
  latitude: 54,
  zoom: 5,
  pitch: 0,
  bearing: 0,
};

const MapPage = () => {
  const [selectedFilter, setSelectedFilter] = useState('');
  const [viewState, setViewState] = useState<any>(INITIAL_VIEW_STATE);
  const [clickedPositions, setClickedPositions] = useState<[number, number][]>([]);
  const [hoveredIconIndex, setHoveredIconIndex] = useState<number | null>(null);

  // Sample data for the map
  const data = [
    { sourcePosition: [-0.118092, 51.509865], targetPosition: [-1.548567, 53.801277] },
    { sourcePosition: [-1.548567, 53.801277], targetPosition: [-2.994362, 53.400002] },
    { sourcePosition: [-2.994362, 53.400002], targetPosition: [-3.188267, 55.953252] },
  ];

  const layers = [
    new TileLayer({
      data: 'https://c.tile.openstreetmap.org/{z}/{x}/{y}.png',
      minZoom: 0,
      maxZoom: 19,
      tileSize: 256,
      renderSubLayers: (props: any) => {
        const {
          bbox: {west, south, east, north}
        } = props.tile;

        return new BitmapLayer(props, {
          data: undefined,
          image: props.data,
          bounds: [west, south, east, north]
        });
      }
    }),
    new LineLayer({
      id: 'line-layer',
      data,
      getSourcePosition: (d: any) => d.sourcePosition,
      getTargetPosition: (d: any) => d.targetPosition,
      getColor: [0, 128, 255],
      getWidth: 2,
    }),
    new IconLayer({
      id: 'icon-layer',
      data: clickedPositions.map((position, index) => ({ position, index })),
      getIcon: () => ({
        url: '/locationpin.png',
        width: 64,
        height: 64,
        anchorX: 32,
        anchorY: 64,
      }),
      getPosition: (d) => d.position,
      getSize: (d) => (d.index === hoveredIconIndex ? 52 : 40),
      sizeScale: 1,
      pickable: true,
      autoHighlight: true,
      onHover: (info) => {
        if (info.object) {
          setHoveredIconIndex(info.object.index);
        } else {
          setHoveredIconIndex(null);
        }
      },
    }),
  ];

  const handleFilterChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    setSelectedFilter(event.target.value);
  };
 
  const handleClickLocation = (info: PickingInfo) => {
    if (info.layer && info.layer.id === 'icon-layer') {
      if (info.object) {
        const clickedIconIndex = info.object.index;
        setClickedPositions((prevPositions) =>
          prevPositions.filter((_, index) => index !== clickedIconIndex)
        );
      }
    } else if (info.coordinate) {
      setClickedPositions((prevPositions) => {
        if (info.coordinate && info.coordinate.length >= 2) {
          const [longitude, latitude] = info.coordinate;
          return [...prevPositions, [longitude, latitude] as [number, number]];
        }
        return prevPositions;
      });
    }
  };

  const handleResetMap = () => {
    setViewState(INITIAL_VIEW_STATE);
    setClickedPositions([]);
  };

  return (
    <div className="flex h-screen">
      <div className="relative w-full">
        <DeckGL
          viewState={viewState}
          onViewStateChange={({ viewState }) => setViewState(viewState)}
          controller={true}
          layers={layers}
          onClick={handleClickLocation}
          getTooltip={({ object }) => object && `Pot of gold: ${object.position}`}
        />
        <button
          className="absolute top-4 right-4 bg-white text-black px-4 py-2 rounded-md shadow-md"
          onClick={handleResetMap}
        >
          Reset Map
        </button>
      </div>
    </div>
  );
};

export default MapPage;
