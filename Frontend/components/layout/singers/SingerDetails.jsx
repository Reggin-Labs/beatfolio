const SongDetails = ({ singerName, subscriber }) => {
  return (
    <div className="flex flex-col justify-center py-1">
      <h3 className="text-2xl font-semibold">
        {singerName ? singerName : 'Song Name'}
      </h3>

      <h4 className="text-xl mt-1 text-gray-500">
        {subscriber ? subscriber : '432k+ Subscribers'}
      </h4>
    </div>
  );
};

export default SongDetails;
