FROM debian:stable
RUN apt-get update && apt-get upgrade -y
RUN mkdir /app
RUN ls /workspace
ADD search_and_sip /app/search_and_sip
CMD /app/search_and_sip
